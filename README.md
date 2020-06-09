# sleep

## About

Windows has no simple tool or command to wait for a certain amount of time while executing a batch file.

And because I already have rust installed on all my computers, I thought it would be fitting to create a simple sleep command which can be installed directly through cargo.

## Installation & Update

With cargo:
```bash
cargo install sleep --force
```

## Command Line
```bash
sleep 1.0
Sleep for a given amount of time

USAGE:
    sleep <TIME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TIME>    The time to sleep in milliseconds
```

## Examples

Sleep for one second:
```bash
sleep 1000
```