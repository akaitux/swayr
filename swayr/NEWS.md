next
=============

- New workspace=__visible__ criterion.

swayr v0.27.0
=============

- New commands `print-config` and `print-default-config`.
- The config can contain a new `[swayr_commands]` section which allows to
  define custom `swaymsg` commands which can then be executed using `swayr
  execute-swaymsg-command`.

swayr v0.26.0
=============

- Improved `get-windows-as-json` with new option `--matching <CRITERIA>` and
  new flag `--error-if-no-match` making it suitable as powerful `swaymsg
  <CRITERIA> nop` replacement in shell scripts.
* `for-each-window <CRITERIA> <SHELL_COMMAND>` executes `<SHELL_COMMAND>` for
  each window matched by `<CRITERIA>`.
* The window format may contain the placeholder `{pid}` which gets replaced by
  the PID.  That's mostly useful for the new `for-each-window` command where
  it's also substituted in the given `<SHELL_COMMAND>` like the other
  placeholders.

swayr v0.25.0
=============

- New criteria `true` and `false`.

swayr v0.24.0
=============

- All commands make the `swayr` binary now exit non-zero if they cannot do
  anything which is great for scripting, e.g., now you can do something like
  `swayr switch-to-app-or-urgent-or-lru-window firefox || firefox` to have a
  "focus firefox or start it, if there's no firefox window already" command.
- New command `get-windows-as-json` returning a JSON array of all windows in
  sway's IPC representation extended by the attributes `swayr_icon` and
  `swayr_type`.

swayr v0.23.0
=============

- New commands `steal-window` and `steal-window-or-container` let you select a
  window (or container) from another workspace using the menu program and move
  it to the current workspace.  Contributed by Rouven Czerwinski.

swayr v0.22.0
=============

- Criteria queries now can have disjunctions (`or`, `OR`, `||`) and negation
  (`NOT`, `!`) in addition to conjunction (implicit or `and`, `AND`, `&&`).

swayr v0.21.0
=============

- There's a new criterion for CRITERIA queries: `workspace=<regex |
  __focused__>` matching windows being on a workspace matching the given
  `regex` or windows being on the same workspace as the focused window (in the
  `__focused__` case).
- New commands `next-matching-window` / `prev-matching-window` which take
  CRITERIA queries defining the windows to be visited.

swayr v0.20.0
=============

- The `switch-to-XXX-or-urgent-or-lru-window` family of commands have had a
  major overhaul.  They used to be plain toggles but now cycle through a
  sequence of all urgent window, all matching windows, the LRU window, and back
  to the original window.
- Swayr now has its own implementation of the CRITERIA API which is used by the
  `switch-to-matching-or-urgent-or-lru-window` command which had been
  introduced in v0.19.0.  Initially, it just passed the query to sway and
  observed if some focus change occurred within a fraction of a second to check
  it's success.  Now it follows the general contract of the
  `switch-to-XXX-or-urgent-or-lru-window` family of commands explained above.
- There's a new `misc.auto_nop_delay` option.
- There's a new `misc.seq_inhibit` option controlling if during a
  `next/prev-window` sequence recording of focus times is deactivated.  This
  option defaults to `false` whereas it used to be hard-coded standard behavior
  in previous swayr versions.

swayr v0.19.0
=============

- There's a new command `switch-to-matching-or-urgent-or-lru-window` which
  switches to the (first) window matching the given criteria (see section
  `CRITERIA` in `sway(5)`) if it exists and is not already focused.  Otherwise,
  switch to the next urgent window (if any) or to the last recently used
  window.

swayr v0.18.0
=============

- The LRU window order will no longer be immediately updated when there is a
  focus change.  Instead there is now a short (configurable) delay
  (`focus.lockin_delay`) before the update.  The user-visible change is that
  quickly moving over windows with the mouse, or moving through them using
  keyboard navigation, will only register the start and destination windows in
  the LRU sequence.
- A `nop` command can be used to interrupt a sequence of window-cycling
  commands.

swayr v0.17.0
=============

- No user-visible changes but a major restructuring and refactoring in order to
  share code between swayr and swayrbar.

swayr v0.16.0
=============

- There's the new command `switch-to-mark-or-urgent-or-lru-window` which
  switches to a specific window matched by mark (`con_mark`) unless it's
  already focused.  In that case, it acts just like
  `switch-to-urgent-or-lru-window`.  For example, you can assign a "browser"
  mark to your browser window (using a standard sway `for_window` rule).  Then
  you can provide "browser" as argument to this command to have a convenient
  browser <-> last-recently-used window toggle.

swayr v0.15.0
=============

- There's a new command `switch-to-app-or-urgent-or-lru-window` which given an
  application ID or window class switches to a matching window unless that's
  already the current window.  In that case, it acts just like
  `switch-to-urgent-or-lru-window`.

swayr v0.14.0
=============

- Instead of just printing everything to stdout and stderr, there's now proper
  logging with timestamps and filtering.  You can define the log level using an
  environment variable like so: `env RUST_LOG=swayr=debug swayrd`.  That would
  start swayr with log level `debug`.  Valid log levels in the order from
  logging more to logging less are: `trace`, `debug`, `info`, `warn`, `error`,
  `off`.

swayr v0.13.0
=============

- All the placeholders except `{app_icon}`, `{indent}`, `{urgency_start}`, and
  `{urgency_end}` may optionally provide a format string as specified by
  [Rust's std::fmt](https://doc.rust-lang.org/std/fmt/).  The syntax is
  `{<placeholder>:<fmt_str><clipped_str>}`.  For example,
  `{app_name:{:>10.10}}` would mean that the application name is printed with
  exactly 10 characters.  If it's shorter, it will be right-aligned (the `>`)
  and padded with spaces, if it's longer, it'll be cut after the 10th
  character.  Another example, `{app_name:{:.10}...}` would mean that the
  application name is truncated at 10 characters.  If it's shorter, it will be
  printed as-is (no padding), if it's longer, it'll be cut after the 10th
  character and the last 3 characters of that substring will be replaced with
  `...` (`<clipped_str>`).

swayr v0.12.0
=============

- The `quit-window` command now has an optional `--kill` / `-k` flag.  If
  given, the process of the window to be quit will be killed using `kill -9
  <pid>` instead of just sending sending the `kill` IPC message to sway.

swayr v0.11.1
=============

- Well, bumping the micro version usually indicates a bugfix release but I've
  forgotten to add the `switch-to` command in version 0.11.0.  It's the
  canonical "switch to anything" command, i.e., it offers outputs, workspaces,
  containers, and windows.

swayr v0.11.0
=============

- New command: `switch-output` shows all outputs in the menu and focuses the
  selected one.  Since outputs must now be printable in the menu program,
  there's a new `format.output_format` spec where you can use the output's
  `{name}` and `{id}` to identify it in the menu program.
- New command: `configure-outputs` lets you repeatedly issue output commands
  until you abort the menu program.
- `move-focused-to` now also supports outputs, i.e., you can move the currently
  focused container to some output which means it's moved to the workspace
  currently active on that output.
- Formats can now include an `{output_name}` placeholder which is replaced by
  the name of the output containing the shown workspace, container or window.

swayr v0.10.0
=============

- The `con` module which enhances the sway IPC container tree structure has
  been replaced by `tree` which achieves the same job but is not restricted to
  only handle workspaces and windows.
- There's a new `format.container_format` for formatting the line showing a
  container.
- Formats such as `format.workspace_format`, `format.container_format`, and
  `format.window_format` can now include a `{indent}` placeholder which will be
  replaced with N times the new `format.indent` value.  N is the depth in the
  shown menu input, e.g., with `swayr switch-workspace-or-window` the indent
  level for workspaces is 0 and 1 for windows.
- The `format.workspace_format` and `format.container_format` may include a
  `{layout}` placeholder which is replaced with the container's layout.
- New command: `switch-workspace-container-or-window` shows workspaces,
  containers, and their windows in the menu program and switches to the
  selected one.
- New command: `quit-workspace-container-or-window` shows workspaces,
  containers, and their windows in the menu program and quits all windows of
  the selected workspace/container or the selected window.
- New command: `swap-focused-with` swaps the currently focused window or
  container with the one selected from the menu program.
- New command: `move-focused-to` moves the currently focused container or
  window to the selected one.  Non-matching input will create a new workspace
  of that name and move the focused container or window there.
  

swayr v0.9.0
============

- The commands `switch-workspace` and `switch-workspace-or-window` now also
  show empty workspaces which makes it possible to switch to another output
  currently showing an empty workspace.
- All menu switching commands (`switch-window`, `switch-workspace`, and
  `switch-workspace-or-window`) now handle non-matching input instead of doing
  nothing.  The input should start with any number of `#` (in order to be able
  to force a non-match), a shortcut followed by a colon, and some string as
  required by the shortcut.  The following shortcuts are supported.
  - `w:<workspace>`: Switches to a possibly non-existing workspace.
    `<workspace>` must be a digit, a name or `<digit>:<name>`.  The
    `<digit>:<name>` format is explained in `man 5 sway`.  If that format is
    given, `swayr` will create the workspace using `workspace number
    <digit>:<name>`.  If just a digit or name is given, the `number` argument
    is not used.
  - `s:<cmd>`: Executes the sway command `<cmd>` using `swaymsg`.
  - Any other input is assumed to be a workspace name and thus handled as
    `w:<input>` would do.
- The command `execute-swaymsg-command` executes non-matching input as
  described by the `s:<cmd>` shortcut above.
- There's a new command `move-focused-to-workspace` which moves the currently
  focused window or container to another workspace selected with the menu
  program.  Non-matching input of the form `#w:<workspace>` where the hash and
  `w:` shortcut are optional can be used to move it to a new workspace.


swayr v0.8.0
============

- There's now the possibility to define a system-wide config file
  `/etc/xdg/swayr/config.toml`.  It is used when no
  `~/.config/swayr/config.toml` exists.
- New commands: `next-tiled-window`, `prev-tiled-window`,
  `next-tabbed-or-stacked-window`, `prev-tabbed-or-stacked-window`,
  `next-floating-window`, `prev-floating-window`, `next-window-of-same-layout`,
  and `prev-window-of-same-layout`.
- **Incompatible change**: All `next/prev-window` commands (including the new
  ones above) now have a mandatory subcommand determining if all or only the
  current workspace's windows should be considered: `all-workspaces` or
  `current-workspace`.
- Bugfix: `prev-window` has never worked correctly.  Instead of cycling through
  all windows in last-recently-used order, it switched between the current and
  last recently used window.  Now it works as expected.
