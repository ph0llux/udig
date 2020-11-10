extern crate clap;
extern crate udev;

use clap::{App, Arg};
use std::io;
use urdig;
use urdig::udev::{Source};
use urdig::traits::*;

fn main() -> io::Result<()> {
	let matches = App::new(urdig::CARGO_PKG_NAME)
		.version(urdig::CARGO_PKG_VERSION)
		.author(urdig::CARGO_PKG_AUTHORS)
		.about(urdig::CARGO_PKG_DESCRIPTION)
		.subcommand(
			App::new(CLAP_SUBCOMMAND_SUBSYSTEMS)
				.about(CLAP_SUBCOMMAND_SUBSYSTEMS_DESCRIPTION)
				.args(&[
					Arg::with_name(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_NAME)
						.short(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_SHORT)
						.long(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_LONG)
						.help(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_DESCRIPTION)
						.takes_value(false)
						.required(false),
					Arg::with_name(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME)
						.short(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME_SHORT)
						.long(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME_LONG)
						.help(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME_DESCRIPTION)
						.takes_value(true)
						.required(false),
					]),
		)
		.subcommand(
			App::new(CLAP_SUBCOMMAND_DEVICE)
				.about(CLAP_SUBCOMMAND_DEVICE_DESCRIPTION)
				.args(&[
					Arg::with_name(CLAP_SUBCOMMAND_DEVICE_ARG_NAME)
						.short(CLAP_SUBCOMMAND_DEVICE_ARG_NAME_SHORT)
						.long(CLAP_SUBCOMMAND_DEVICE_ARG_NAME_LONG)
						.help(CLAP_SUBCOMMAND_DEVICE_ARG_NAME_DESCRIPTION)
						.index(1)
						.required(true),
					Arg::with_name(CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES)
						.short(CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES_SHORT)
						.long(CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES_LONG)
						.help(CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES_DESCRIPTION)
						.takes_value(false)
						.required(false),
					Arg::with_name(CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES)
						.short(CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES_SHORT)
						.long(CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES_LONG)
						.help(CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES_DESCRIPTION)
						.takes_value(false)
						.required(false),
					Arg::with_name(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT)
						.short(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_SHORT)
						.long(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_LONG)
						.help(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_DESCRIPTION)
						.default_value(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_DEFAULT_VALUE)
						.possible_values(&CLAP_POSSIBLE_FORMAT_VALUES)
						.takes_value(true)
						.required(false),
				]),
		)
		.get_matches();
	//SUBSYSTEMS
	if let Some(matches) = matches.subcommand_matches(CLAP_SUBCOMMAND_SUBSYSTEMS) {
		if matches.is_present(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_NAME) {
			subsystems(SubsystemAction::ListAll)?;
			return Ok(());
		} else if matches.is_present(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME) {
			let devnode = matches.value_of(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME).to_io_result()?;
			subsystems(SubsystemAction::DevnodesPerSubsystem(devnode.to_string()))?;
			return Ok(());
		}
	//DEVICE
	} else if let Some(matches) = matches.subcommand_matches(CLAP_SUBCOMMAND_DEVICE) {
		if matches.is_present(CLAP_SUBCOMMAND_DEVICE_ARG_NAME) {
			let devicename = matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_NAME).to_io_result()?;
			if matches.is_present(CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES) {
				let format = {
					if matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT).to_io_result()?.to_lowercase() == CLAP_FORMAT_VALUE_LISTING {
						Format::Listing
					} else if matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT).to_io_result()?.to_lowercase() == CLAP_FORMAT_VALUE_JSON {
						Format::Json
					} else if matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT).to_io_result()?.to_lowercase() == CLAP_FORMAT_VALUE_TOML {
						Format::Toml
					} else {
						println!("{}", urdig::ERROR_FORMAT_UNKNOWN);
						return Ok(());
					}
				};
				if devicename.starts_with(urdig::DEV) {
					print_device_properties(devicename, format, Source::Devnode)?;
				} else if devicename.starts_with(urdig::SYS) {
					print_device_properties(devicename, format, Source::Syspath)?;
				} else {
					print_device_properties(devicename, format, Source::Sysname)?;
				}
			};
			if matches.is_present(CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES) {
				let format = {
					if matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT).to_io_result()?.to_lowercase() == CLAP_FORMAT_VALUE_LISTING {
						Format::Listing
					} else if matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT).to_io_result()?.to_lowercase() == CLAP_FORMAT_VALUE_JSON {
						Format::Json
					} else if matches.value_of(CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT).to_io_result()?.to_lowercase() == CLAP_FORMAT_VALUE_TOML {
						Format::Toml
					} else {
						println!("{}", urdig::ERROR_FORMAT_UNKNOWN);
						return Ok(());
					}
				};
				if devicename.starts_with(urdig::DEV) {
					print_device_attributes(devicename, format, Source::Devnode)?;
				} else if devicename.starts_with(urdig::SYS) {
					print_device_attributes(devicename, format, Source::Syspath)?;
				} else {
					print_device_attributes(devicename, format, Source::Sysname)?;
				}
			};
			if !matches.is_present(CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES) && !matches.is_present(CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES) {
				println!("{}", urdig::ERROR_NO_PROPERTIES_AND_ATTRIBUTES);
			}
		}
		return Ok(());
	};
	Ok(())
}

enum SubsystemAction {
	ListAll,
	DevnodesPerSubsystem(String),
}

enum Format {
	Listing,
	Json,
	Toml
}

fn subsystems(action: SubsystemAction) -> io::Result<()> {
	match action {
		SubsystemAction::ListAll => {
			for subsystem in urdig::udev::get_subsystems()? {
				println!("{}", subsystem);
			}
			Ok(())
		},
		SubsystemAction::DevnodesPerSubsystem(x) => {
			for devnode in urdig::udev::get_nodes_from_subsystem(x)? {
				println!("{}", devnode);
			}
			Ok(())
		},
	}
}

fn print_device_properties<S: Into<String>>(name: S, format: Format, source: Source) -> io::Result<()> {
	let name = name.into();
	match format {
		Format::Listing => {
			println!("{} {} {}", urdig::PROPERTY_VALUE_SYSNAME, urdig::SEPARATOR_COLON, name);
			for (name, value) in urdig::udev::get_properties(name, source)? {
				println!("{} : {}", name, value);
			}
			return Ok(());
		}
		Format::Json => {
			println!("{}", urdig::SEPARATOR_BRACE_OPEN);
			println!("\t\"{}\" {} {}", urdig::PROPERTY_VALUE_SYSNAME, urdig::SEPARATOR_COLON, urdig::SEPARATOR_BRACE_OPEN);
			for (name, value) in urdig::udev::get_properties(name, source)? {
				println!("\t\t\"{}\" {} \"{}\"", name, urdig::SEPARATOR_COLON, value);
			}
			println!("\t{}", urdig::SEPARATOR_BRACE_CLOSE);
			println!("{}", urdig::SEPARATOR_BRACE_CLOSE);
			return Ok(());
		}
		Format::Toml => {
			println!("{}{}{}", urdig::SEPARATOR_SQUARE_BRACKET_OPEN, name, urdig::SEPARATOR_SQUARE_BRACKET_CLOSE);
			for (name, value) in urdig::udev::get_properties(name, source)? {
				println!("{}{}\"{}\"", name, urdig::SEPARATOR_EQUAL, value);
			}
			return Ok(());
		}
	}
}

fn print_device_attributes<S: Into<String>>(name: S, format: Format, source: Source) -> io::Result<()> {
	let name = name.into();
	match format {
		Format::Listing => {
			println!("{} {} {}", urdig::PROPERTY_VALUE_SYSNAME, urdig::SEPARATOR_COLON, name);
			for (name, value) in urdig::udev::get_attributes(name, source)? {
				println!("{} : {}", name, value);
			}
			return Ok(());
		}
		Format::Json => {
			println!("{}", urdig::SEPARATOR_BRACE_OPEN);
			println!("\t\"{}\" {} {}", urdig::PROPERTY_VALUE_SYSNAME, urdig::SEPARATOR_COLON, urdig::SEPARATOR_BRACE_OPEN);
			for (name, value) in urdig::udev::get_attributes(name, source)? {
				println!("\t\t\"{}\" {} \"{}\"", name, urdig::SEPARATOR_COLON, value);
			}
			println!("\t{}", urdig::SEPARATOR_BRACE_CLOSE);
			println!("{}", urdig::SEPARATOR_BRACE_CLOSE);
			return Ok(());
		}
		Format::Toml => {
			println!("{}{}{}", urdig::SEPARATOR_SQUARE_BRACKET_OPEN, name, urdig::SEPARATOR_SQUARE_BRACKET_CLOSE);
			for (name, value) in urdig::udev::get_attributes(name, source)? {
				println!("{}{}\"{}\"", name, urdig::SEPARATOR_EQUAL, value);
			}
			return Ok(());
		}
	}
}


//- clap values
//- - Subcommand SUBSYSTEMS:
const CLAP_SUBCOMMAND_SUBSYSTEMS: &str = "subsystems";
const CLAP_SUBCOMMAND_SUBSYSTEMS_DESCRIPTION: &str = "interacting with udev subsystems.";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_NAME: &str = "list-all";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_SHORT: &str = "l";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_LONG: &str = "list-all";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_DESCRIPTION: &str = "prints all available udev subsystems.";

const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME: &str = "subsystem-name";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME_SHORT: &str = "n";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME_LONG: &str = "subsystem-name";
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_SUBSYSTEMNAME_DESCRIPTION: &str = "prints all available devnodes from given subsystem-name (if subsystem exists).";

//----------------------------------------------

const CLAP_SUBCOMMAND_DEVICE: &str = "device";
const CLAP_SUBCOMMAND_DEVICE_DESCRIPTION: &str = "print options for specific device, which is called via its name. You can also use devnodes or syspaths.";

const CLAP_SUBCOMMAND_DEVICE_ARG_NAME: &str = "device name";
const CLAP_SUBCOMMAND_DEVICE_ARG_NAME_SHORT: &str = "n";
const CLAP_SUBCOMMAND_DEVICE_ARG_NAME_LONG: &str = "name";
const CLAP_SUBCOMMAND_DEVICE_ARG_NAME_DESCRIPTION: &str = "Specifies the 'device name', which could be used. Could be the sysname, the devnode or the syspath. Will be detected automatically.";

const CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES: &str = "properties";
const CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES_SHORT: &str = "p";
const CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES_LONG: &str = "properties";
const CLAP_SUBCOMMAND_DEVICE_ARG_PROPERTIES_DESCRIPTION: &str = "If set, all properties of the given device will be shown";

const CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT: &str = "format";
const CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_SHORT: &str = "f";
const CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_LONG: &str = "format";
const CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_DEFAULT_VALUE: &str = "listing";
const CLAP_SUBCOMMAND_DEVICE_ARG_FORMAT_DESCRIPTION: &str = "Set the output format. Default format is \"listing\".";

const CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES: &str = "attributes";
const CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES_SHORT: &str = "a";
const CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES_LONG: &str = "attributes";
const CLAP_SUBCOMMAND_DEVICE_ARG_ATTRIBUTES_DESCRIPTION: &str = "If set, all attributes of the given device will be shown";


const CLAP_FORMAT_VALUE_LISTING: &str = "listing";
const CLAP_FORMAT_VALUE_JSON: &str = "json";
const CLAP_FORMAT_VALUE_TOML: &str = "toml";
const CLAP_POSSIBLE_FORMAT_VALUES: [&str; 3] = [ CLAP_FORMAT_VALUE_LISTING, CLAP_FORMAT_VALUE_JSON, CLAP_FORMAT_VALUE_TOML ];

//----------------------------------------------