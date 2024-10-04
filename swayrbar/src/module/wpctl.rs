// Copyright (C) 2024  Tassilo Horn <tsdh@gnu.org>
// Copyright (C) 2024  bitraid <bitraid@protonmail.ch>
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.

//! The wpctl `swayrbar` module.

use crate::config;
use crate::module::{BarModuleFn, RefreshReason};
use crate::shared::fmt::subst_placeholders;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;
use swaybar_types as s;

const NAME: &str = "wpctl";

struct State {
    volume: u8,
    muted: bool,
    volume_source: u8,
    muted_source: bool,
    cached_text: String,
}

pub static VOLUME_RX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r".* (?<num>\d+)\.(?<frac>\d{2}).*").unwrap());

fn run_wpctl(args: &[&str]) -> String {
    match Command::new("wpctl").args(args).output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(err) => {
            log::error!("Could not run wpctl: {err}");
            String::new()
        }
    }
}

fn get_volume(device: &str) -> (u8, bool) {
    let output = run_wpctl(&["get-volume", device]);
    let mut volume = String::new();
    if !output.is_empty() {
        VOLUME_RX
            .captures(&output)
            .unwrap()
            .expand("$num$frac", &mut volume);
    }
    (volume.parse::<u8>().unwrap_or(255_u8), output.contains("[MUTED]"))
}

pub struct BarModuleWpctl {
    config: config::ModuleConfig,
    state: Mutex<State>,
}

fn refresh_state(state: &mut State, fmt_str: &str, html_escape: bool) {
    (state.volume, state.muted) = get_volume("@DEFAULT_AUDIO_SINK@");
    (state.volume_source, state.muted_source) = get_volume("@DEFAULT_AUDIO_SOURCE@");
    state.cached_text = subst_placeholders(fmt_str, html_escape, state);
}

fn subst_placeholders(fmt: &str, html_escape: bool, state: &State) -> String {
    subst_placeholders!(fmt, html_escape, {
        "volume" => {
            state.volume
        },
        "muted" =>{
            if state.muted {
                " muted"
            } else {
                ""
            }
        },
        "volume_source" => {
            state.volume_source
        },
        "muted_source" =>{
            if state.muted_source {
                " muted"
            } else {
                ""
            }
        },
    })
}

pub fn create(config: config::ModuleConfig) -> Box<dyn BarModuleFn> {
    Box::new(BarModuleWpctl {
        config,
        state: Mutex::new(State {
            volume: 255_u8,
            muted: false,
            volume_source: 255_u8,
            muted_source: false,
            cached_text: String::new(),
        }),
    })
}

impl BarModuleFn for BarModuleWpctl {
    fn default_config(instance: String) -> config::ModuleConfig
    where
        Self: Sized,
    {
        config::ModuleConfig {
            name: NAME.to_owned(),
            instance,
            format: "🔈 Vol: {volume:{:3}}%{muted}".to_owned(),
            html_escape: Some(true),
            on_click: Some(HashMap::from([
                ("Left".to_owned(),
                 vec!["foot".to_owned(), "watch".to_owned(),
                 "wpctl".to_owned(), "status".to_owned()]),
                (
                    "Right".to_owned(),
                    vec![
                        "wpctl".to_owned(),
                        "set-mute".to_owned(),
                        "@DEFAULT_AUDIO_SINK@".to_owned(),
                        "toggle".to_owned(),
                    ],
                ),
                (
                    "WheelUp".to_owned(),
                    vec![
                        "wpctl".to_owned(),
                        "set-volume".to_owned(),
                        "@DEFAULT_AUDIO_SINK@".to_owned(),
                        "1%+".to_owned(),
                    ],
                ),
                (
                    "WheelDown".to_owned(),
                    vec![
                        "wpctl".to_owned(),
                        "set-volume".to_owned(),
                        "@DEFAULT_AUDIO_SINK@".to_owned(),
                        "1%-".to_owned(),
                    ],
                ),
            ])),
        }
    }

    fn get_config(&self) -> &config::ModuleConfig {
        &self.config
    }

    fn build(&self, reason: &RefreshReason) -> s::Block {
        let mut state = self.state.lock().expect("Could not lock state.");

        if match reason {
            RefreshReason::TimerEvent => true,
            RefreshReason::ClickEvent { name, instance } => {
                name == &self.config.name && instance == &self.config.instance
            }
            _ => false,
        } {
            refresh_state(
                &mut state,
                &self.config.format,
                self.config.is_html_escape(),
            );
        }

        s::Block {
            name: Some(NAME.to_owned()),
            instance: Some(self.config.instance.clone()),
            full_text: state.cached_text.to_owned(),
            align: Some(s::Align::Left),
            markup: Some(s::Markup::Pango),
            short_text: None,
            color: None,
            background: None,
            border: None,
            border_top: None,
            border_bottom: None,
            border_left: None,
            border_right: None,
            min_width: None,
            urgent: None,
            separator: Some(true),
            separator_block_width: None,
        }
    }

    fn subst_cmd_args<'a>(&'a self, cmd: &'a [String]) -> Vec<String> {
        let state = self.state.lock().expect("Could not lock state.");
        cmd.iter()
            .map(|arg| subst_placeholders(arg, false, &state))
            .collect()
    }
}
