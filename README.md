# Swayr & Swayrbar

[![builds.sr.ht status](https://builds.sr.ht/~tsdh/swayr.svg)](https://builds.sr.ht/~tsdh/swayr?)
[![License GPL 3 or later](https://img.shields.io/crates/l/swayr.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![dependency status](https://deps.rs/repo/sourcehut/~tsdh/swayr/status.svg)](https://deps.rs/repo/sourcehut/~tsdh/swayr)
[![Hits-of-Code](https://hitsofcode.com/sourcehut/~tsdh/swayr?branch=main)](https://hitsofcode.com/sourcehut/~tsdh/swayr/view?branch=main)

## Table of Contents

* [Swayr](#swayr)
  * [Commands](#swayr-commands)
  * [Screenshots](#swayr-screenshots)
  * [Installation](#swayr-installation)
  * [Usage](#swayr-usage)
  * [Configuration](#swayr-configuration)
  * [Version changes](#swayr-version-changes)
* [Swayrbar](#swayrbar)
  * [Screenshots](#swayrbar-screenshots)
  * [Installation](#swayrbar-installation)
  * [Configuration](#swayrbar-configuration)
  * [Version changes](#swayrbar-version-changes)
* [Questions and patches](#questions-and-patches)
* [Bugs](#bugs)
* [Build status](#build-status)
* [License](#license)

## <a id="swayr">Swayr, a window-switcher & more for [sway](https://swaywm.org/)</a>

[![latest release](https://img.shields.io/crates/v/swayr.svg)](https://crates.io/crates/swayr)

> **Note on terminology:** When starting this project, I somehow confused the
> terms most-recently-used (MRU) and least-recently-used (LRU).  I've meant the
> former but called it last-recently-used whose meaning is the same as
> most-recently-used but whose abbreviation LRU is actually the opposite.  So
> now there are many commands with `lru` in their name: it always means
> most-recently-used (MRU), not least-recently-used (LRU).  It's too late to
> rename the commands.  Maybe I'll add aliases at some point and deprecate the
> old names...

Swayr consists of a daemon, and a client.  The `swayrd` daemon records
window/workspace creations, deletions, and focus changes using sway's JSON IPC
interface.  The `swayr` client offers subcommands, see `swayr --help`, and
sends them to the daemon which executes them.

### <a id="swayr-commands">Swayr commands</a>

The `swayr` binary provides many subcommands of different categories.

#### <a id="swayr-non-menu-switchers">Non-menu switchers</a>

Those are commands which switch through a sequence of windows where the
sequence is:
1. All windows with urgency hints.
2. All matching windows where which windows match is specific to the command.
3. The most recently used window at the time of the sequence start.
4. Back to the origin window, i.e., the window which had the focus at the time
   of the sequence start.

During each sequence no window will be visited twice, e.g., if some window has
an urgency hint, matches the commands specification, and is also the LRU
window, it's not visited once in each step 1, 2, and 3 but just in step 1.

The steps 1, 3, and 4 can be inhibited with the flags `--skip-urgent`,
`--skip-lru`, and `--skip-origin`, respectively.

As said, which windows match is specific to each command:

* `switch-to-urgent-or-lru-window` matches nothing, so step 2 above is
  effectively disabled.
* `switch-to-app-or-urgent-or-lru-window <name>` matches windows with the
  specified name.  The name is compared literally against the window's `app_id`
  for native Wayland windows or to the window class or instance for X11
  windows.  The command immediately exits non-zero if there is no matching
  window at all.
* `switch-to-mark-or-urgent-or-lru-window <con_mark>` matches the window having
  the given mark.  As `man sway(5)` defines, each mark can only be applied to a
  single window at a time.  The command immediately exits non-zero if there is
  no matching window at all.
* `switch-to-matching-or-urgent-or-lru-window <criteria>` matches windows
  according to the the given [criteria query](#swayr-commands-criteria).  The
  command immediately exits non-zero if there is no matching window at all.

All above commands also have a flag `--skip-lru-if-current-doesnt-match` which
is like `--skip-lru` but skips the LRU window only if the currently focused
window is no matching window (by app name, mark, or criteria).  Note that
`switch-to-urgent-or-lru-window` has this flag for purely technical reasons but
it has no effect there.

The `switch-to-app-or-urgent-or-lru-window` can be conveniently used to define
switch-to-or-start commands for your favorite applications, e.g., I have those:

```sh
bindsym $mod+e exec \
        swayr switch-to-app-or-urgent-or-lru-window \
              --skip-lru-if-current-doesnt-match emacs \
        || emacs
bindsym $mod+b exec \
        swayr switch-to-app-or-urgent-or-lru-window \
              --skip-lru-if-current-doesnt-match firefoxdeveloperedition \
        || firefox-developer-edition
```

#### Menu switchers

Those spawn a menu program where you can select a window (or workspace, or
output, etc.) and act on that.

* `switch-window` displays all windows in the order of urgent windows first,
  then windows in most-recently-used order, and the currently focused window
  last.  The window selected in the menu program will be focused.
* `steal-window` displays all windows in the order or `switch-window` and moves
   the chosen window into the current workspace.
* `steal-window-or-container` displays all windows and containers moves the
   window or container into the current workspace.
* `switch-workspace` displays all workspaces in MRU order and switches to the
  selected one.
* `switch-output` shows all outputs in the menu and focuses the selected one.
* `switch-workspace-or-window` displays all workspaces and their windows and
   switches to the selected workspace or window.
* `switch-workspace-container-or-window` shows workspaces, containers, and
  their windows in the menu program and switches to the selected one.
* `switch-to` shows outputs, workspaces, containers, and their windows in the
  menu program and switches to the selected one.
* `quit-window` displays all windows and quits the selected one.  An optional
  `--kill` / `-k` flag may be specified in which case the window's process will
  be killed using `kill -9 <pid>` rather than only sending a `kill` IPC message
  to sway.
* `quit-workspace-or-window` displays all workspaces and their windows and
  allows to quit either the selected workspace (all its windows) or the
  selected window.
* `quit-workspace-container-or-window` shows workspaces, containers, and their
  windows and quits all windows of the selected workspace/container or the
  selected window.
* `move-focused-to-workspace` moves the currently focused window or container
  to another workspace selected with the menu program.  Non-matching input of
  the form `#w:<workspace>` where the hash and `w:` shortcut are optional can
  be used to move it to a new workspace.
* `move-focused-to` moves the currently focused container or window to the
  selected output, workspace, container, window.  Non-matching input is handled
  like with `move-focused-to-workspace`.
* `swap-focused-with` swaps the currently focused window or container with the
  one selected from the menu program.

##### Menu shortcuts for non-matching input

All menu switching commands (`switch-window`, `switch-workspace`, and
`switch-workspace-or-window`) now handle non-matching input instead of doing
nothing.  The input should start with any number of `#` (in order to be able to
force a non-match), a shortcut followed by a colon, and some string as required
by the shortcut.  The following shortcuts are supported.
- `w:<workspace>`: Switches to a possibly non-existing workspace.
  `<workspace>` must be a digit, a name or `<digit>:<name>`.  The
  `<digit>:<name>` format is explained in `man 5 sway`.  If that format is
  given, `swayr` will create the workspace using `workspace number
  <digit>:<name>`.  If just a digit or name is given, the `number` argument is
  not used.
- `s:<cmd>`: Executes the sway command `<cmd>` using `swaymsg`.
- Any other input is assumed to be a workspace name and thus handled as
  `w:<input>` would do.


#### <a id="swayr-cycling-commands">Cycling commands</a>

Those commands cycle through (a subset of windows) in most-recently-used order.

* `next-window (all-workspaces|current-workspace)` & `prev-window
  (all-workspaces|current-workspace)` focus the next/previous window in
  depth-first iteration order of the tree.  The argument `all-workspaces` or
  `current-workspace` define if all windows of all workspaces or only those of
  the current workspace are considered.
* `next-tiled-window` & `prev-tiled-window` do the same as `next-window` &
  `prev-window` but switch only between windows contained in a tiled container.
* `next-tabbed-or-stacked-window` & `prev-tabbed-or-stacked-window` do the same
  as `next-window` & `prev-window` but switch only between windows contained in
  a tabbed or stacked container.
* `next-floating-window` & `prev-floating-window` do the same as `next-window`
  & `prev-window` but switch only between floating windows.
* `next-window-of-same-layout` & `prev-window-of-same-layout` is like
  `next-floating-window` / `prev-floating-window` if the current window is
  floating, it is like `next-tabbed-or-stacked-window` /
  `prev-tabbed-or-stacked-window` if the current window is in a tabbed or
  stacked container, it is like `next-tiled-window` / `prev-tiled-window` if
  the current windows is in a tiled container, and is like `next-window` /
  `prev-window` otherwise.
* `next-matching-window` / `prev-matching-window` both take a [criteria
  query](#swayr-commands-criteria).

#### Layout modification commands

These commands change the layout of the current workspace.

* `tile-workspace exclude-floating|include-floating` tiles all windows on the
  current workspace (excluding or including floating ones).  That's done by
  moving all windows away to some special workspace, setting the current
  workspace to `splith` layout, and then moving the windows back.  If the
  `auto_tile` feature is used, see the Configuration section below, it'll
  change from splitting horizontally to vertically during re-insertion.
* `shuffle-tile-workspace exclude-floating|include-floating` shuffles & tiles
  all windows on the current workspace.  The shuffle part means that (a) the
  windows are shuffled before re-insertion, and (b) a randomly chosen already
  re-inserted window is focused before re-inserting another window.  So while
  `tile-workspace` on a typical horizontally oriented screen and 5 windows will
  usually result in a layout with one window on the left and all four others
  tiled vertially on the right, `shuffle-tile-workspace` in combination with
  `auto_tile` usually results in a more balanced layout, i.e., 2 windows tiled
  vertically on the right and the other 4 tiled vertially on the left.  If you
  have less than a handful of windows, just repeat `shuffle-tile-workspace` a
  few times until happenstance creates the layout you wanted.
* `tab-workspace exclude-floating|include-floating` puts all windows of the
  current workspace into a tabbed container.
* `toggle-tab-shuffle-tile-workspace exclude-floating|include-floating` toggles
  between a tabbed and tiled layout, i.e., it calls `shuffle-tile-workspace` if
  it is currently tabbed, and calls `shuffle-tile-workspace` if it is currently
  tiled.

#### Scripting commands

* `get-windows-as-json` returns a JSON containing all windows, possibly with
  scratchpad windows if `--include-scratchpad` is given.  Furthermore,
  `--matching <CRITERIA>` can be used to restrict the windows to those matching
  the given criteria query (see [the criteria
  section](#swayr-commands-criteria)).  Lastly, if `--error-if-no-match` is
  given and no windows exist or match the given criteria query, the command
  exits non-zero instead of printing a JSON array.  This makes it suitable for
  shell scripting.  Essentially, `swayr get-windows-as-json --matching
  <CRITERIA> --error-if-no-match` is like `swaymsg <CRITERIA> nop` except that
  it returns the windows as JSON and support's swayr's extended criteria
  queries instead of the simple ones supported by sway.
* `for-each-window <CRITERIA> <SHELL_COMMAND>` executes `<SHELL_COMMAND>` for
  each window matched by `<CRITERIA>` (see [the criteria
  section](#swayr-commands-criteria)).  In `<SHELL_COMMAND>` almost all
  placeholders defined in [the section about window
  formats](#swayr-window-placeholders) are replaced.  For example, `swayr
  for-each-window true echo "The app {app_name} has the PID {pid}."` tells the
  application name and the pid for each window.  The result of the command is a
  JSON array with objects containing the exit code, stdout, stderr, and a
  (system) error field.  If any command returns non-zero, so will
  `for-each-window`.  The shell commands will be executed in parallel they must
  finish within 2 seconds, otherwise they'll be killed.  Otherwise, the command
  execution would block `swayrd` for as long as the slowest thread requires,
  e.g., `sleep 10` would block for slightly over 10 seconds.

#### Miscellaneous commands

* `configure-outputs` lets you repeatedly issue output configuration commands
  until you abort the menu program.
* `execute-swaymsg-command` displays most swaymsg which don't require
  additional input and executes the selected one.  That's handy especially for
  less often used commands not bound to a key.  Non-matching input will be
  executed executed as-is with `swaymsg`.  Also note that custom commands can
  be defined in the [configuration file](#swayr-configuration)'s
  `[swaymsg_commands]` section.
* `execute-swayr-command` displays all commands above and executes the selected
  one.  (This is useful for accessing swayr commands which are not bound to a
  key.)
* `nop` (unsurprisingly) does nothing, the command can be used to break out of
  a sequence of [non-menu switching commands](#swayr-non-menu-switchers) or
  [window cycling commands](#swayr-cycling-commands).  The LRU window order is
  frozen when the first cycling command is processed and remains so until a
  non-cycling command is received.  The `nop` command can conveniently serve to
  interrupt a sequence without having any other side effects.

#### <a id="swayr-commands-criteria">Criteria</a>

Swayr supports most of the criteria querys defined by Sway, see section
`CRITERIA` in `man sway(5)`.  Right now, these are:
* `app_id=<regex | __focused__>`
* `class=<regex | __focused__>`
* `instance=<regex | __focused__>`
* `title=<regex | __focused__>`
* `workspace=<regex | __focused__ | __visible__ >`
* `con_mark=<regex>`
* `con_id=<uint | __focused__>`
* `shell=<"xdg_shell" | "xwayland" | __focused__>`
* `pid=<uint>`
* `floating`
* `tiling`
* `app_name=<regex | __focused__>` (not in sway!)

The last criterion `app_name` is matched against the application's name which
can either be `app_id`, `window_properties.class`, or
`window_properties.instance` (whatever is filled).

All regular expressions are [Rust's regex crates
regexes](https://docs.rs/regex/latest/regex/index.html).  With the special
value `__focused__`, comparison is performed literally.

In addition to the simple criteria listed above, criteria queries can be
combined using `and`, `or`, and `not` with the syntax:
* `[and <crit1> <crit2> ...]` which is equivalent to `[<crit1> <crit2> ...]`,
  i.e., the `and` is optional for compatibility with sway which only supports
  this syntax and has no `or` and `not`.  `[and]` and `[]` always match.
* `[or <crit1> <crit2> ...]` where `[or]` never matches.
* `not <crit>` where the following criterion is negated.

The combinators may also be written in all-caps, i.e., `AND`, `OR`, and `NOT`,
or as `&&`, `||`, and `!`.

Obviously, criteria may be nested, so this is a valid one:
```
[|| [app_id="firefox" tiling]
    [&& !app_id="firefox" floating workspace=__focused__]]
```

There are also the boolean literals `true` and `false` available which may also
be written in all-caps.


### <a id="swayr-screenshots">Screenshots</a>

![A screenshot of swayr switch-window](misc/switch-window.png "swayr
switch-window")

![A screenshot of swayr
switch-workspace-or-window](misc/switch-workspace-or-window.png "swayr
switch-workspace-or-window")

### <a id="swayr-installation">Installation</a>

Some distros have packaged swayr so that you can install it using your distro's
package manager.  Alternatively, it's easy to build and install it yourself
using `cargo`.

#### Distro packages

The following GNU/Linux and BSD distros package swayr.  Thanks a lot to the
respective package maintainers!  Refer to the [repology
site](https://repology.org/project/swayr/versions) for details.

[![Packaging status](https://repology.org/badge/vertical-allrepos/swayr.svg)](https://repology.org/project/swayr/versions)
[![AUR swayr-git package status](https://repology.org/badge/version-for-repo/aur/swayr.svg?allow_ignored=yes&header=AUR%20swayr-git)](https://repology.org/project/swayr/versions)

#### Building with cargo

You'll need to install the current stable rust toolchain using the one-liner
shown at the [official rust installation
page](https://www.rust-lang.org/tools/install).

Then you can install swayr like so:
```sh
cargo install swayr
```

For getting updates easily, I recommend the cargo `cargo-update` plugin.
```sh
# Install it once.
cargo install cargo-update

# Then you can update all installed rust binary crates including swayr using:
cargo install-update --all

# If you only want to update swayr, you can do so using:
cargo install-update -- swayr
```

### <a id="swayr-usage">Usage</a>

You need to start the swayr daemon (`swayrd`) in your sway config
(`~/.config/sway/config`) like so:

```
exec env RUST_BACKTRACE=1 RUST_LOG=swayr=debug swayrd > /tmp/swayrd.log 2>&1
```

The setting of `RUST_BACKTRACE=1`, `RUST_LOG=swayr=debug` and the redirection
of the output to some logfile is optional but helps a lot when something
doesn't work.  Especially, if you encounter a crash in certain situations and
you want to report a bug, it would be utmost helpful if you could reproduce the
issue with backtrace and logging at the `debug` level and attach that to your
bug report.  Valid log levels in the order from logging more to logging less
are: `trace`, `debug`, `info`, `warn`, `error`, `off`.

Beyond starting the daemon, you will want to bind swayr commands to some keys
like so:

```
bindsym $mod+Space       exec swayr switch-window
bindsym $mod+Delete      exec swayr quit-window
bindsym $mod+Tab         exec swayr switch-to-urgent-or-lru-window
bindsym $mod+Next        exec swayr next-window all-workspaces
bindsym $mod+Prior       exec swayr prev-window all-workspaces
bindsym $mod+Shift+Space exec swayr switch-workspace-or-window
bindsym $mod+c           exec swayr execute-swaymsg-command
bindsym $mod+Shift+c     exec swayr execute-swayr-command
```

Of course, configure the keys to your liking.

Pending a fix for [Sway issue
#6456](https://github.com/swaywm/sway/issues/6456) or a merge of [Sway PR
#6920](https://github.com/swaywm/sway/pull/6920), it will be possible to close
a sequence of [non-menu switching commands](#swayr-non-menu-switchers) or
[window cycling commands](#swayr-cycling-commands) using a `nop` command bound
to the release of the `$mod` key.  Assuming your `$mod` is bound to `Super_L`
it could look something like this:

```
bindsym --release Super_L exec swayr nop
```

Until then, there's the `focus.auto_nop_delay` option which see below in the
[Configuration](#swayr-configuration) section.


### <a id="swayr-configuration">Configuration</a>

Swayr can be configured using the `~/.config/swayr/config.toml` or
`/etc/xdg/swayr/config.toml` config file.

If no config files exists, a simple default configuration will be created on the
first invocation for use with the [wofi](https://todo.sr.ht/~scoopta/wofi)
menu program.

It should be easy to adapt that default config for usage with other menu
programs such as [fuzzel](https://codeberg.org/dnkl/fuzzel),
[dmenu](https://tools.suckless.org/dmenu/),
[bemenu](https://github.com/Cloudef/bemenu),
[rofi](https://github.com/davatorium/rofi), a script spawning a terminal with
[fzf](https://github.com/junegunn/fzf), or whatever.  The only requirement is
that the launcher needs to be able to read the items to choose from from stdin
and spit out the selected item to stdout.

The default config looks like this:

```toml
[menu]
executable = 'wofi'
args = [
    '--show=dmenu',
    '--define=layer=overlay',
    '--allow-markup',
    '--allow-images',
    '--insensitive',
    '--cache-file=/dev/null',
    '--parse-search',
    '--height=40%',
    '--prompt={prompt}',
]

[format]
output_format = '{indent}<b>Output {name}</b>    <span alpha=\"20000\">({id})</span>'
workspace_format = '{indent}<b>Workspace {name} [{layout}]</b>    <span alpha="20000">({id})</span>'
container_format = '{indent}<b>Container [{layout}]</b> on workspace {workspace_name} <i>{marks}</i>    <span alpha="20000">({id})</span>'
window_format = 'img:{app_icon}:text:{indent}<i>{app_name}</i> — {urgency_start}<b>“{title}”</b>{urgency_end} on workspace {workspace_name} <i>{marks}</i>    <span alpha="20000">({id})</span>'
indent = '    '
urgency_start = '<span background="darkred" foreground="yellow">'
urgency_end = '</span>'
html_escape = true
icon_dirs = [
    '/usr/share/icons/hicolor/scalable/apps',
    '/usr/share/icons/hicolor/64x64/apps',
    '/usr/share/icons/hicolor/48x48/apps',
    '/usr/share/icons/Adwaita/64x64/apps',
    '/usr/share/icons/Adwaita/48x48/apps',
    '/usr/share/pixmaps',
]

[layout]
auto_tile = false
auto_tile_min_window_width_per_output_width = [
    [1024, 500],
    [1280, 600],
    [1400, 680],
    [1440, 700],
    [1600, 780],
    [1920, 920],
    [2560, 1000],
    [3440, 1000],
    [4096, 1200],
]

[focus]
lockin_delay = 750

[misc]
auto_nop_delay = 3000
seq_inhibit = false

[swaymsg_commands]
include_predefined = true
[swaymsg_commands.commands]
"Window to workspace XXX" = "move window to workspace XXX"
"Workspace to left output" = "move workspace to output left"
"Workspace to right output" = "move workspace to output right"
```

In the following, all sections are explained.

#### The menu section

In the `[menu]` section, you can specify the menu program using the
`executable` name or full path and the `args` (flags and options) it should get
passed.  If some argument contains the placeholder `{prompt}`, it is replaced
with a prompt such as "Switch to window" depending on context.

#### <a id="swayr-window-placeholders">The format section</a>

In the `[format]` section, format strings are specified defining how selection
choices are to be layed out.  `wofi` supports [pango
markup](https://docs.gtk.org/Pango/pango_markup.html) which makes it possible
to style the text using HTML and CSS.  The following formats are supported
right now.
* `output_format` defines how outputs (monitors) are displayed in the menu
  program, `workspace_format` defines how workspaces are displayed,
  `container_format` defines how non-workspace containers are displayed, and
  `window_format` defines how application windows are displayed.
* In these formats, the following placeholders can be used:
  * `{name}` gets replaced by the output name, the workspace number or name or
    a window's title.  The placeholder `{title}` is an obsolete synonym which
    will be removed in a later version.
  * `{layout}` shows the workspace or container's layout.
  * `{id}` gets replaced by the sway-internal con id.
  * `{pid}` gets replaced by the PID.
  * `{indent}` gets replaced with N times the new `format.indent` value where N
    is the depth in the shown menu input.
  * `{app_name}` gets replaced with a window's application name.
  * `{marks}` shows a comma-separated list of the container's or window's
     marks.
  * `{app_icon}` shows the application's icon (a path to a PNG or SVG file).
  * `{workspace_name}` gets replaced with the name or number of the workspace
    the container or window belongs to.
  * The placeholders `{urgency_start}` and `{urgency_end}` get replaced by the
    empty string if the window has no urgency flag and with the values of the
    same-named formats if the window has the urgency flag set.  That makes it
    possible to highlight urgent windows as shown in the default config.
* `indent` is a string which is repeatedly inserted at the `{indent}`
  placeholder in formats.
* `html_escape` defines if the strings replacing the placeholders above (except
  for `{urgency_start}` and `{urgency_end}`) should be HTML-escaped.
* `urgency_start` is a string which replaces the `{urgency_start}` placeholder
  in `window_format`.
* `urgency_end` is a string which replaces the `{urgency_end}` placeholder in
  `window_format`.
* `icon_dirs` is a vector of directories in which to look for application icons
  in order to compute the `{app_icon}` replacement.
* `fallback_icon` is a path to some PNG/SVG icon which will be used as
  `{app_icon}` if no application-specific icon can be determined.

All the <a id="fmt-placeholders">placeholders</a> except `{app_icon}`,
`{indent}`, `{urgency_start}`, and `{urgency_end}` may optionally provide a
format string as specified by [Rust's
std::fmt](https://doc.rust-lang.org/std/fmt/).  The syntax is
`{<placeholder>:<fmt_str><clipped_str>}`.  For example, `{app_name:{:>10.10}}`
would mean that the application name is printed with exactly 10 characters.  If
it's shorter, it will be right-aligned (the `>`) and padded with spaces, if
it's longer, it'll be cut after the 10th character.  Another example,
`{app_name:{:.10}...}` would mean that the application name is truncated at 10
characters.  If it's shorter, it will be printed as-is (no padding), if it's
longer, it'll be cut after the 10th character and the last 3 characters of that
substring will be replaced with `...` (`<clipped_str>`).

It is crucial that during selection (using wofi or some other menu program)
each window has a different display string.  Therefore, it is highly
recommended to include the `{id}` placeholder at least in `container_format`
and `window_format`.  Otherwise, e.g., two vertical splits on the same
workspace or two terminals (of the same terminal app) with the same working
directory (and therefore, the same title) wouldn't be distinguishable.

**Hint for wofi**: `wofi` supports icons with the syntax
`'img:<image-file>:text:<text>'`, so a suitable `window_format` with
application icon should start with `img:{app_icon}:text:`.

**Hint for rofi**: `rofi` supports icons with the syntax
`"<text>\u0000icon\u001f<image-file>"`, so a suitable window_format with
application icon should end with `"\u0000icon\u001f{app_icon}"`.  Also note
that you must enclose your `window_format` value with double-quotes and not
with single-quotes.  Singe-quote strings are literal strings in
[TOML](https://toml.io/en/v1.0.0#string) where no escape-sequences are
processed whereas for double-quoted strings (so-called basic strings)
escape-sequences are processed.  `rofi` requires a null character and a
PARAGRAPH SEPARATOR for image sequences.

**Hint for fuzzel**: I've been told that `fuzzel` supports the very same icon
syntax as discussed for `rofi` above.

#### The layout section

In the `[layout]` section, you can enable auto-tiling by setting `auto_tile` to
`true` (the default is `false`).  The option
`auto_tile_min_window_width_per_output_width` defines the minimum width in
pixels which your windows should have per output width.  For example, the
example setting above says that on an output which is 1600 pixels wide, each
window should have at least a width of 780 pixels, thus there may be at most
two side-by-side windows (Caution, include your borders and gaps in your
calculation!).  There will be no auto-tiling doesn't include your output's
exact width.

If `auto_tile` is enabled, swayr will automatically split either vertically or
horizontally according to this algorithm:
- For all outputs:
  + For all (nested) containers on that output (except the scratchpad):
    - For all child windows of that container:
      + If the container is split horizontally and creating another window
        would make the current child window smaller than the minimum width,
        execute `split vertical` (the `swaymsg` command over IPC) on the child.
      + Else if the container is split vertically and now there is enough space
        so that creating another window would still leave the current child
        window above or equal to the minimum width, call `split horizontal` on
        the child.
      + Otherwise, do nothing for this container.  This means that stacked or
        tabbed containers will never be affected by auto-tiling.

There is one caveat: it would be nice to also trigger auto-tiling when windows
or containers are resized but unfortunately, resizing doesn't issue any events
over IPC.  Therefore, auto-tiling is triggered by new-window events,
close-events, move-events, floating-events, and also focus-events.  The latter
are a workaround and wouldn't be required if there were resize-events.


#### The focus section

In the `[focus]` section, you can configure the amount of time a window has to
keep the focus in order to affect the LRU order, the `lockin_delay` (specified
in milliseconds).  If a given window is only briefly focused, e.g., by moving
the mouse over it on the way to another window with sway's
`focus_follows_mouse` set to `yes` or `always`, then its position in the LRU
order will not be modified.


#### The misc section

In the `[misc]` section, there's the `auto_nop_delay` option.  When some swayr
command is executed, this amount of milliseconds is waited before a `nop`
command (see the [commands documentation](#swayr-commands)) is executed in
order to break out of a `next-*-window`/`prev-*-window` sequence or a
`switch-to-*or-urgent-or-lru-window` cycle automatically.  If another swayr
command is executed within this time frame, the auto-`nop` execution will be
delayed for another `auto_nop_delay` milliseconds.  If this option is not
specified explicitly, no automatic `nop` commands will be executed.

A more elegant solution using a key release binding is discussed at the end of
the [Usage](#swayr-usage) section.  However, that requires a PR to sway which
has not been merged so far.

The `seq_inhibit` boolean controls how `swayrd` behaves during a _sequence_ of
[window cycling commands](#swayr-cycling-commands).

- When the setting is `true`, `swayrd` will inhibit updates to the window LRU
  order while a _sequence_ of window cycling commands is in progress.  LRU
  updates are reactivated when the _sequence_ ends.  A _sequence_ is considered
  to have ended when any non-window-cycling-command is received by `swayrd`
  (e.g. a `nop` command).

  Note: LRU update inhibition also applies to focus changes made outside of
  `swayr`, for instance by using sway commands directly.

- When the setting is `false` (the default), `swayrd` will handle focus events
  the same way regardless of whether a window cycling sequence is in progress
  or not.

Note that the key release binding solution lends itself to using
`seq_inhibit=true`.

#### The swaymsg_commands section

This section configures the `execute-swaymsg-command` command.

- The option `include_predefined` defines if the default swaymsg commands,
  which swayr provided for a long time, should be included.
- The `commands` hashmap defines your custom commands as `label = command`
  pairs.  Since it's a map, the labels (keys) need to be unique.

### <a id="swayr-version-changes">Version changes</a>

Since version 0.8.0, I've started writing a [NEWS](swayr/NEWS.md) file listing the
news, and changes to `swayr` commands or configuration options.  If something
doesn't seem to work as expected after an update, please consult this file to
check if there has been some (possibly incompatible) change requiring an update
of your config.

## <a id="swayrbar">Swayrbar</a>

[![latest release](https://img.shields.io/crates/v/swayrbar.svg)](https://crates.io/crates/swayrbar)

`swayrbar` is a status command for sway's `swaybar` implementing the
[`swaybar-protocol(7)`](https://man.archlinux.org/man/swaybar-protocol.7).
This means, you would setup your `swaybar` like so in your
`~/.config/sway/config`:

```conf
bar {
    swaybar_command swaybar
    # Use swayrbar as status command with some logging output which
    # is redirected to /tmp/swayrbar.log.  Be sure to only redirect
    # stderr because the swaybar protocol requires the status_command
    # to emit JSON to stdout which swaybar reads.
    status_command env RUST_BACKTRACE=1 RUST_LOG=swayr=debug swayrbar 2> /tmp/swayrbar.log
    position top
    font pango:Iosevka 11
    height 20

    colors {
        statusline #f8c500
        background #33333390
    }
}
```

`swayrbar`, like [waybar](https://github.com/Alexays/Waybar/), consists of a
set of modules which you can enable and configure via its config file, either
the one specified via the command line option `--config-file`, the
user-specific (`~/.config/swayrbar/config.toml`), or the system-wide
(`/etc/xdg/swayrbar/config.toml`).  Modules emit information which `swaybar`
then displays and mouse clicks on a module's space in `swaybar` are propagated
back and trigger some action (e.g., a shell command).

Right now, there are the following modules:

1. The `window` module can show the title and application name of the current
   window in sway.
2. The `sysinfo` module can show things like CPU/memory utilization or system
   load.
3. The `battery` module can show the current [state of
   charge](https://en.wikipedia.org/wiki/State_of_charge), the state (e.g.,
   charging), and the [state of
   health](https://en.wikipedia.org/wiki/State_of_health).
4. The `date` module can show, you guess it, the current date and time!
5. The `pactl` module can show the current volume percentage and muted state.
   Clicks can increase/decrease the volume or toggle the mute state.  It
   requires PulseAudio and can also control PipeWire if the pipewire-pulse
   adapter library is installed.
6. The `wpctl` module can show the current volume percentage and muted state.
   Clicks can increase/decrease the volume or toggle the mute state. It
   requires PipeWire.
7. The `nmcli` module uses NetworkManager's `nmcli` command line tool to show
   the currently connected wifi and its signal strength.
8. The `iwctl` module the `iwctl` command line tool to show the currently
   connected wifi and its signal strength.


I guess there will be more modules in the future as time permits.
[Patches](#questions-and-patches) are certainly very welcome!

### <a id="swayrbar-screenshots">Screenshots</a>

![A screenshot of swaybar running with swayrbar](misc/swayrbar.png "swaybar
with swayrbar")

### <a id="swayrbar-installation">Installation</a>

Some distros have a swayrbar package so that you can install it using your
distro's package manager, see the [repology
site](https://repology.org/project/swayrbar/versions) for details.
Alternatively, it's easy to build and install it yourself using `cargo`.

[![Packaging status](https://repology.org/badge/vertical-allrepos/swayrbar.svg)](https://repology.org/project/swayrbar/versions)


#### Installation via Cargo

You'll need to install the current stable rust toolchain using the one-liner
shown at the [official rust installation
page](https://www.rust-lang.org/tools/install).

Then you can install swayrbar like so:
```sh
cargo install swayrbar
```

For getting updates easily, I recommend the cargo `install-update` plugin.
```sh
# Install it once.
cargo install install-update

# Then you can update all installed rust binary crates including swayr using:
cargo install-update --all

# If you only want to update swayr, you can do so using:
cargo install-update -- swayrbar
```


### <a id="swayrbar-configuration">Configuration</a>

When `swayrbar` is run for the very first time and doesn't find an existing
configuration file at `~/.config/swayrbar/config.toml` (user-specific) or
`/etc/xdg/swayrbar/config.toml` (system-wide), it'll create a new user-specific
one where all modules are enabled and set up with some reasonable (according to
the author) default values.  Adapt it to your needs.

The syntax of the config file is [TOML](https://toml.io/en/).  Here's a short
example with all top-level options (one!) and one module.

```toml
refresh_interval = 1000

[[modules]]
name = 'window'
instance = '0'
format = '🪟 {title} — {app_name}'
html_escape = false

[modules.on_click]
Left = ['swayr', 'switch-to-urgent-or-lru-window']
Right = ['kill', '{pid}']
```

The `refresh_interval` defines the number of milliseconds between refreshes of
`swaybar`.

The remainder of the configuration defines a list of modules with their
configuration (which is an [array of
tables](https://toml.io/en/v1.0.0#array-of-tables) in TOML where a module's
`on_click`).

* `name` is the name or type of the module, e.g., `window`, `sysinfo`,
  `battery`, `date`,...
* `instance` is an arbitrary string used for distinguishing two modules of the
  same `name`.  For example, you might want to have two `sysinfo` modules, one
  for CPU and one for memory utilization, simply to have a separator between
  these different kinds of information.  That's easily doable, just give them
  different `instance` values.
* `format` is the string to be printed in `swaybar` where certain placeholders
  are substituted with module-specific values.  Usually, such placeholders are
  written like `{title}`, i.e., inside braces.  Like in `swayr`, formatting
  (padding, aligning, precision, etc.) is available, see
  [here](#fmt-placeholders).
* `html_escape` defines if `<`, `>`, and `&` should be escaped as `&lt;`,
  `&gt;`, and `&amp;` because `format` may contain [pango
  markup](https://docs.gtk.org/Pango/pango_markup.html).  Obviously, if you
  make use of this feature, you want to set `html_escape = true` for that
  module.  This option is optional and may be omitted which has the same
  meaning as setting it to `false`.
* `on_click` is a table defining shell commands to be performed when you
  click on a module's space in `swaybar`.  All placeholders available in
  `format` are available here, too.  The action for each mouse button is
  specified as an array `['command', 'arg1', 'arg2',...]`.  The
  available button names to be assigned to are `Left`, `Middle`,
  `Right`, `WheelUp`, `WheelDown`, `WheelLeft`, and `WheelRight`.

The `on_click` table can also be written as inline table

```toml
on_click = { Left = ['swayr', 'switch-to-urgent-or-lru-window'], Right = ['kill', '{pid}'] }
```

but then it has to be on one single line.


#### The `window` module

The `window` module supports the following placeholders:
* `{title}` or `{name}` expand to the currently focused window's title.
* `{app_name}` is the application name.
* `{pid}` is the process id.

Note that the `window` module also reacts to title change events of windows
which are not current and that's a feature!  For examle, consider your Emacs on
workspace 1 is focused.  Now you click a link in there which causes your
Firefox on (the invisible) workspace 2 to open a new tab.  This will cause
`swayrbar` to display the Firefox title so you can see that your click had an
effect.  After at most 3 seconds, the title of the focused application will be
displayed again.

By default, it has the following click bindings:
* `Left` executes `swayr switch-to-urgent-or-lru-window`.
* `Right` kills the process of the window.


#### The `sysinfo` module

The `sysinfo` module supports the following placeholders:
* `{cpu_usage}` is the percentage of CPU utilization.
* `{mem_usage}` is the percentage of memory utilization.
* `{load_avg_1}` is the average system load in the last minute.
* `{load_avg_5}` is the average system load in the last five minutes.
* `{load_avg_15}` is the average system load in the last fifteen minutes.

By default, it has the following click bindings:
* `Left` executes `foot htop`.


#### The `battery` module

The `battery` module supports the following placeholders:
* `{state_of_charge}` is the percentage of charge wrt. the battery's current
  capacity.
* `{state_of_health}` is the percentage of the battery's remaining capacity
  compared to its original capacity.
* `{state}` is the current state, e.g., something like Discharging or Full.


#### The `pactl` module

The `pactl` module requires the pulse-audio command line tool of the same name
to be installed.  It supports the following placeholders:
* `{volume}` is the current volume percentage of the default sink.
* `{muted}` is the string `" muted"` if the default sink is currently muted,
  otherwise it is the empty string.
* `{volume_source}` is the current volume percentage of the default source.
* `{muted_source}` is the string `" muted"` if the default source is currently
  muted, otherwise it is the empty string.

By default, it has the following click bindings:
* `Left` calls the `pavucontrol` program (PulseAudio GUI control).
* `Right` toggles the default sink's mute state.
* `WheelUp` and `WheelDown` increase/decrease the volume of the default sink.


#### The `wpctl` module

The `wpctl` module requires the pipewire command line tool of the same name
to be installed.  It supports the following placeholders:
* `{volume}` is the current volume percentage of the default sink.
* `{muted}` is the string `" muted"` if the default sink is currently muted,
  otherwise it is the empty string.
* `{volume_source}` is the current volume percentage of the default source.
* `{muted_source}` is the string `" muted"` if the default source is currently
  muted, otherwise it is the empty string.

By default, it has the following click bindings:
* `Left` executes `foot watch wpctl status` (monitor PipeWire objects).
* `Right` toggles the default sink's mute state.
* `WheelUp` and `WheelDown` increase/decrease the volume of the default sink.


#### The `nmcli` module

The `nmcli` module requires NetworkManager and the `nmcli` command line tool.
It can display information about the wifi connection.  It supports the
following placeholders:
* `{name}` wifi network name.
* `{signal}` wireless signal strength (in %).
* `{bars}` a visualization of connection strength, like "▂▄▆_".

#### The `iwctl` module

The `iwctl` module requires the `iwctl` command line tool which comes with
`iwd`.  It can display information about the wifi connection.  It supports the
following placeholders:
* `{name}` wifi network name.
* `{signal}` wireless signal strength (in dBm).
* `{bars}` a visualization of connection strength, like "▂▄▆_".

#### The `date` module

The `date` module shows the date and time by defining the `format` using
[chrono's strftime
format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers).

#### The `cmd` module

The `cmd` module can be used to run shell commands and display their
output.

The command is specified using the `format` configuration option.  It will be
executed by `sh -c` and its output will be displayed in the module's space in
`swaybar`.

The `html_escape` option controls if HTML entity replacements should be
performed on the command's output before it is displayed in the bar, i.e., you
should set it to `true` if your command outputs text containing `<`, `>`, or
`&` but is not valid pango markup.

This module has no placeholders or default configuration.

### <a id="swayr-version-changes">Version changes</a>

Version changes are summarized in the [NEWS](swayrbar/NEWS.md) file.  If
something doesn't seem to work as expected after an update, please consult this
file to check if there has been some (possibly incompatible) change requiring
an update of your config.


## <a id="questions-and-patches">Questions & Patches</a>

For asking questions, sending feedback, or patches, refer to [my public inbox
(mailinglist)](https://lists.sr.ht/~tsdh/public-inbox).  Please mention the
project you are referring to in the subject, e.g., `swayr` or `swayrbar` (or
other projects in different repositories).

## <a id="bugs">Bugs</a>

It compiles, therefore there are no bugs.  Oh well, if you still found one or
want to request a feature, you can do so
[here](https://todo.sr.ht/~tsdh/swayr).

## <a id="build-status">Build status</a>

[![builds.sr.ht status](https://builds.sr.ht/~tsdh/swayr.svg)](https://builds.sr.ht/~tsdh/swayr?)

## <a id="license">License</a>

Swayr & Swayrbar are licensed under the
[GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html) (or later).
