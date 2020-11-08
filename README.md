# udig
The **u**dev **r**ecognized **d**evices **i**nformations **g**rabber.

## Installation

### Build from source

#### Pre-requirements

You need the udev development files, which could be installed by following commands (at the according distributions):

| Distribution | Installation command      |
|--------------|---------------------------|
| Fedora       | dnf install systemd-devel |
| Ubuntu       | apt install libudev-dev   |
| Debian       | apt install lubudev-dev   |

You need also a rust compiler and cargo. The most suitable way to install it, is [using rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### building manually

You could download the tool manually and build it with cargo:
```bash
git clone https://github.com/ph0llux/udig
cd udig
cargo build --release
```
The binary can be find at ```./target/release/udig```.

## Usage
You can print the help menu by using
```bash
$ ./target/debug/udig --help
udig 0.1.0
ph0llux <ph0llux@pm.me>
Grab device informations via libudev and display or parse them.

USAGE:
    udig [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help          Prints this message or the help of the given subcommand(s)
    subsystems    interacting with udev subsystems.
    sysname       print options for specific device, which is called via its sysname.
```
