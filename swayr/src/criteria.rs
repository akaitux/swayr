// Copyright (C) 2022-2023  Tassilo Horn <tsdh@gnu.org>
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

//! Implementation of sway's criteria API.

use crate::{shared::ipc, shared::ipc::NodeMethods, tree as t};
use regex::Regex;
use swayipc as s;

#[derive(Debug)]
pub enum RegexOrFocused {
    Regex(Regex),
    Focused,
}

#[derive(Debug)]
pub enum RegexOrFocusedOrVisible {
    Regex(Regex),
    Focused,
    Visible,
}

#[derive(Debug)]
pub enum I64OrFocused {
    I64(i64),
    Focused,
}

#[derive(Debug)]
pub enum ShellTypeOrFocused {
    ShellType(s::ShellType),
    Focused,
}

#[derive(Debug)]
pub enum Criterion {
    // And/Or/Not aren't specified by sway.
    And(Vec<Criterion>),
    Or(Vec<Criterion>),
    Not(Box<Criterion>),
    BoolLiteral(bool),
    AppId(RegexOrFocused),
    Class(RegexOrFocused),
    Instance(RegexOrFocused),
    /// Not specified by sway: matched against either app_id or class,
    /// depending on if the window is a wayland or X11 window.
    AppName(RegexOrFocused),
    Title(RegexOrFocused),
    ConMark(Regex),
    ConId(I64OrFocused),
    Pid(i32),
    Workspace(RegexOrFocusedOrVisible),
    Shell(ShellTypeOrFocused),
    Floating,
    Tiling,
}

fn regex_from_str(s: &str) -> Regex {
    match Regex::new(s) {
        Ok(rx) => rx,
        Err(err) => {
            log::error!("Invalid regex {s:?}: {err}");
            Regex::new("^__I_WONT_MATCH_A_💩__$").unwrap()
        }
    }
}

peg::parser! {
    grammar criteria_parser() for str {
        rule space() -> () = [' ' | '\t']* {}
        rule i32_literal() -> i32 =
            n:$(['-']?['0'..='9']+) {? n.parse().or(Err("i32")) }
        rule i64_literal() -> i64 =
            n:$(['-']?['0'..='9']+) {? n.parse().or(Err("i64")) }
        rule string_literal() -> String =
            "\"" s:[^'"']* "\"" { s.into_iter().collect() }

        rule regex_or_focused() -> RegexOrFocused =
            "__focused__" { RegexOrFocused::Focused }
          / s:string_literal() { RegexOrFocused::Regex(regex_from_str(&s)) }
        rule regex_or_focused_or_visible() -> RegexOrFocusedOrVisible =
            "__focused__" { RegexOrFocusedOrVisible::Focused }
          / "__visible__" { RegexOrFocusedOrVisible::Visible }
          / s:string_literal() { RegexOrFocusedOrVisible::Regex(regex_from_str(&s)) }

        rule i64_focused() -> I64OrFocused =
            "__focused__" { I64OrFocused::Focused }
        rule i64_or_focused() -> I64OrFocused =
            i64_focused() / n:i64_literal() { I64OrFocused::I64(n) }

        rule tiling() -> Criterion = "tiling" { Criterion::Tiling }
        rule floating() -> Criterion = "floating" { Criterion::Floating }
        rule app_id() -> Criterion = "app_id" space() "=" space()
            rof:regex_or_focused() { Criterion::AppId(rof) }
        rule app_name() -> Criterion = "app_name" space() "=" space()
            rof:regex_or_focused() { Criterion::AppName(rof) }
        rule class() -> Criterion = "class" space() "=" space()
            rof:regex_or_focused() { Criterion::Class(rof) }
        rule instance() -> Criterion = "instance" space() "=" space()
            rof:regex_or_focused() { Criterion::Instance(rof) }
        rule title() -> Criterion = "title" space() "=" space()
            rof:regex_or_focused() { Criterion::Title(rof) }
        rule con_mark() -> Criterion = "con_mark" space() "=" space()
            s:string_literal() { Criterion::ConMark(regex_from_str(&s)) }
        rule con_id() -> Criterion = "con_id" space() "=" space()
            i:i64_or_focused() { Criterion::ConId(i) }
        rule pid() -> Criterion = "pid" space() "=" space()
            n:i32_literal() { Criterion::Pid(n) }
        rule workspace() -> Criterion = "workspace" space() "=" space()
            rof:regex_or_focused_or_visible() { Criterion::Workspace(rof) }
        rule shell_type_or_focused() -> ShellTypeOrFocused =
            "\"xdg_shell\"" {ShellTypeOrFocused::ShellType(s::ShellType::XdgShell)}
          / "\"xwayland\""  {ShellTypeOrFocused::ShellType(s::ShellType::Xwayland)}
          / "__focused__"   {ShellTypeOrFocused::Focused}
        rule shell() -> Criterion = "shell" space() "=" space()
            stof:shell_type_or_focused() { Criterion::Shell(stof) }

        rule and() -> Criterion =
            "[" space() ("AND" / "and" / "&&")? space()
                l:(criterion() ** space())
                space() "]" space()
            { Criterion::And(l) }

        rule or() -> Criterion =
            "[" space() ("OR" / "or" / "||") space()
                l:(criterion() ** space())
                space() "]" space()
            { Criterion::Or(l) }

        rule not() -> Criterion =
            ("NOT" / "not" / "!") space() c:criterion() space()
            { Criterion::Not(Box::new(c)) }

        rule bool_literal() -> Criterion =
            ("TRUE" / "true") { Criterion::BoolLiteral(true) }
          / ("FALSE" / "false") { Criterion::BoolLiteral(false) }

        rule criterion() -> Criterion =
            and() / or() / not()
          / bool_literal()
          / tiling() / floating()
          / app_id() / class() / instance() / app_name() / title() / shell()
          / workspace()
          / con_mark()
          / con_id()
          / pid()

        pub rule parse() -> Criterion =
            space() c:criterion() space()
        { c }
  }
}

pub fn parse_criteria(criteria: &str) -> Result<Criterion, String> {
    criteria_parser::parse(criteria).map_err(|e| e.to_string())
}

fn is_some_and_rx_matches(s: Option<&String>, rx: &Regex) -> bool {
    s.is_some() && rx.is_match(s.unwrap())
}

fn are_some_and_equal<T: std::cmp::PartialEq>(
    a: Option<T>,
    b: Option<T>,
) -> bool {
    a.is_some() && b.is_some() && a.unwrap() == b.unwrap()
}

fn eval_criterion<'a>(
    criterion: &'a Criterion,
    w: &'a t::DisplayNode,
    focused: Option<&'a t::DisplayNode>,
) -> bool {
    match criterion {
        Criterion::And(criteria) => {
            criteria.iter().all(|crit| eval_criterion(crit, w, focused))
        }
        Criterion::Or(criteria) => {
            criteria.iter().any(|crit| eval_criterion(crit, w, focused))
        }
        Criterion::Not(crit) => !eval_criterion(crit, w, focused),
        Criterion::BoolLiteral(val) => *val,
        Criterion::AppId(val) => match val {
            RegexOrFocused::Regex(rx) => {
                is_some_and_rx_matches(w.node.app_id.as_ref(), rx)
            }
            RegexOrFocused::Focused => match focused {
                Some(win) => are_some_and_equal(
                    w.node.app_id.as_ref(),
                    win.node.app_id.as_ref(),
                ),
                None => false,
            },
        },
        Criterion::AppName(val) => match val {
            RegexOrFocused::Regex(rx) => rx.is_match(w.node.get_app_name()),
            RegexOrFocused::Focused => match focused {
                Some(win) => w.node.get_app_name() != win.node.get_app_name(),
                None => false,
            },
        },
        Criterion::Class(val) => match val {
            RegexOrFocused::Regex(rx) => is_some_and_rx_matches(
                w.node
                    .window_properties
                    .as_ref()
                    .and_then(|wp| wp.class.as_ref()),
                rx,
            ),
            RegexOrFocused::Focused => match focused {
                Some(win) => are_some_and_equal(
                    w.node
                        .window_properties
                        .as_ref()
                        .and_then(|p| p.class.as_ref()),
                    win.node
                        .window_properties
                        .as_ref()
                        .and_then(|p| p.class.as_ref()),
                ),
                None => false,
            },
        },
        Criterion::Instance(val) => match val {
            RegexOrFocused::Regex(rx) => is_some_and_rx_matches(
                w.node
                    .window_properties
                    .as_ref()
                    .and_then(|wp| wp.instance.as_ref()),
                rx,
            ),
            RegexOrFocused::Focused => match focused {
                Some(win) => are_some_and_equal(
                    w.node
                        .window_properties
                        .as_ref()
                        .and_then(|p| p.instance.as_ref()),
                    win.node
                        .window_properties
                        .as_ref()
                        .and_then(|p| p.instance.as_ref()),
                ),
                None => false,
            },
        },
        Criterion::Shell(val) => match val {
            ShellTypeOrFocused::ShellType(t) => {
                w.node.shell.as_ref() == Some(t)
            }
            ShellTypeOrFocused::Focused => match focused {
                Some(win) => are_some_and_equal(
                    w.node.shell.as_ref(),
                    win.node.shell.as_ref(),
                ),
                None => false,
            },
        },
        Criterion::ConId(val) => match val {
            I64OrFocused::I64(id) => w.node.id == *id,
            I64OrFocused::Focused => w.node.focused,
        },
        Criterion::ConMark(rx) => w.node.marks.iter().any(|m| rx.is_match(m)),
        Criterion::Pid(pid) => w.node.pid == Some(*pid),
        Criterion::Workspace(val) => match val {
            RegexOrFocusedOrVisible::Regex(rx) => {
                let ws_name = w
                    .tree
                    .get_parent_node_of_type(w.node.id, ipc::Type::Workspace)
                    .map(|ws| ws.get_name().to_owned());
                is_some_and_rx_matches(ws_name.as_ref(), rx)
            }
            RegexOrFocusedOrVisible::Focused => match focused {
                Some(win) => are_some_and_equal(
                    w.tree.get_parent_node_of_type(
                        w.node.id,
                        ipc::Type::Workspace,
                    ),
                    win.tree.get_parent_node_of_type(
                        win.node.id,
                        ipc::Type::Workspace,
                    ),
                ),
                None => false,
            },
            RegexOrFocusedOrVisible::Visible => {
                if let Some(ws) = w
                    .tree
                    .get_parent_node_of_type(w.node.id, ipc::Type::Workspace)
                {
                    if let Some(output) =
                        w.tree.get_parent_node_of_type(ws.id, ipc::Type::Output)
                    {
                        return output
                            .focus
                            .first()
                            .is_some_and(|&visible| visible == ws.id);
                    }
                }
                false
            }
        },
        Criterion::Floating => w.node.is_floating(),
        Criterion::Tiling => !w.node.is_floating(),
        Criterion::Title(val) => match val {
            RegexOrFocused::Regex(rx) => {
                is_some_and_rx_matches(w.node.name.as_ref(), rx)
            }
            RegexOrFocused::Focused => match focused {
                Some(win) => are_some_and_equal(
                    w.node.name.as_ref(),
                    win.node.name.as_ref(),
                ),
                None => false,
            },
        },
    }
}

pub fn criterion_to_predicate<'a>(
    criterion: &'a Criterion,
    all_windows: &'a [t::DisplayNode],
) -> impl Fn(&t::DisplayNode) -> bool + 'a {
    let focused = all_windows.iter().find(|x| x.node.focused);
    move |w: &t::DisplayNode| eval_criterion(criterion, w, focused)
}

#[test]
fn test_criteria_parser() {
    match criteria_parser::parse(
        "[tiling floating app_id=__focused__ app_id=\"foot\" class=\"emacs\" instance = \"the.instance\" title=\"something with :;&$\" con_mark=\"^.*foo$\"\tapp_name=\"Hugo\" con_id = __focused__ con_id=17 pid=23223 shell=\"xdg_shell\" shell=\"xwayland\" shell=__focused__ workspace=\"test\" workspace=__focused__ true false TRUE FALSE]",
    ) {
        Ok(c) => assert!(matches!(c, Criterion::And(..))),
        Err(err) => {
            unreachable!("Could not parse: {}", err);
        },
    }
}

#[test]
fn test_criteria_parser_and() {
    for c in ["[]", "[and]", "[AND]", "[&&]"] {
        match criteria_parser::parse(c) {
            Ok(c) => {
                println!("Criteria: {:?}", c);
                assert!(match c {
                    Criterion::And(v) => v.is_empty(),
                    _ => false,
                })
            }
            Err(err) => {
                unreachable!("Could not parse: {}", err);
            }
        }
    }
}

#[test]
fn test_criteria_parser_or() {
    for c in ["[or]", "[OR]", "[||]"] {
        match criteria_parser::parse(c) {
            Ok(c) => {
                println!("Criteria: {:?}", c);
                assert!(match c {
                    Criterion::Or(v) => v.is_empty(),
                    _ => false,
                })
            }
            Err(err) => {
                unreachable!("Could not parse: {}", err);
            }
        }
    }
}

#[test]
fn test_criteria_parser_not() {
    for c in ["not tiling", "NOT tiling", "!tiling", "! tiling"] {
        match criteria_parser::parse(c) {
            Ok(c) => {
                println!("Criteria: {:?}", c);
                assert!(match c {
                    Criterion::Not(x) =>
                        matches!(x.as_ref(), Criterion::Tiling),
                    _ => false,
                })
            }
            Err(err) => {
                unreachable!("Could not parse: {}", err);
            }
        }
    }
}
