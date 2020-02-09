# `tnb` - Telegram Notification Bot

`$ some_command | tnb` - will send the output of the command to your [Telegram](https://telegram.org/) conversation of choice.

Useful for streaming the output of long-running commands, when you don't want/can't keep a constant SSH connection or you
only need a notification when something finishes.

# Installation
## Arch/Manjaro
`tnb` package on the AUR.

## Others
`cargo build --release`, then copy the resulting binary (at `./target/release/tnb`) to some folder in your PATH.
