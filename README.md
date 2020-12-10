# urdig
The **u**dev **r**ecognized **d**evices **i**nformation **g**rabber.

## Installation

### Use pre-compiled binaries

#### ... install as snap

[![Get it from the Snap Store](https://snapcraft.io/static/images/badges/en/snap-store-black.svg)](https://snapcraft.io/urdig)

### Build from source

#### Pre-requirements

You need the udev development files, which could be installed by following commands (at the according distributions):

| Distribution | Installation command      |
|--------------|---------------------------|
| Fedora       | dnf install systemd-devel |
| Ubuntu       | apt install libudev-dev   |
| Debian       | apt install libudev-dev   |

You need also a rust compiler and cargo. The most suitable way to install it, is [using rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### building manually

You could download the tool manually and build it with cargo:
```bash
git clone https://github.com/ph0llux/urdig
cd urdig
cargo build --release
```
The binary can be find at ```./target/release/urdig```.

## Usage
You can print the help menu by using
```bash
$ urdig 
urdig 0.9.3
ph0llux <ph0llux@pm.me>
Grab device informations via libudev and display or parse them.

USAGE:
    urdig [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    device        print options for specific device, which is called via its name. You can also use devnodes or
                  syspaths.
    help          Prints this message or the help of the given subcommand(s)
    subsystems    interacting with udev subsystems.
```
and grab informations of any device
```
$ urdig device -p /dev/sda
[output omitted]
```
