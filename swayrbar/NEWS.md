swayrbar 0.4.1
==============

- New `wpctl` module (similar to the `pactl` module) for adjusting volume of
  the default source and sink using `wpctl` (from PipeWire).

swayrbar 0.4.0
==============

- New `cmd` module that runs a user-specified shell command and displays its
  output.
- The `pactl` module now also supports the default source (microphone).

swayrbar 0.3.1
==============

- The `nmcli` module has been generalized to support the `iwctl` tool, too.
  Just specify one of these two names as the module name.

swayrbar 0.3.0
==============

- There's a new `nmcli` wifi module contributed by Karl Eklund.

swayrbar 0.2.0
==============

- If a window module is used, subscribe to sway events in order to immediately
  refresh it on window/workspace changes.

swayrbar 0.1.1
==============

- Only refresh the module which received the click event.

swayrbar 0.1.0
==============

- Add pactl module.
