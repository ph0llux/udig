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
			App::new(CLAP_SUBCOMMAND_SYSNAME)
				.about(CLAP_SUBCOMMAND_SYSNAME_DESCRIPTION)
				.args(&[
					Arg::with_name(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME)
						.short(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_SHORT)
						.long(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_LONG)
						.help(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_DESCRIPTION)
						.takes_value(true)
						.required(true),
					Arg::with_name(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES)
						.short(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_SHORT)
						.long(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_LONG)
						.help(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_DESCRIPTION)
						.takes_value(false)
						.required(false),
					Arg::with_name(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES)
						.short(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_SHORT)
						.long(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_LONG)
						.help(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_DESCRIPTION)
						.takes_value(false)
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
	//SYSNAME
	} else if let Some(matches) = matches.subcommand_matches(CLAP_SUBCOMMAND_SYSNAME) {
		if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME) {
			let devicename = matches.value_of(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME).to_io_result()?;
			println!("{} {} {}", urdig::PROPERTY_VALUE_SYSNAME, urdig::SEPARATOR_COLON, devicename);
			if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES) {
				let format = Format::Listing; //TODO
				if devicename.starts_with(urdig::DEV) {
					print_properties(devicename, format, Source::Devnode)?;
				} else if devicename.starts_with(urdig::SYS) {
					print_properties(devicename, format, Source::Syspath)?;
				} else {
					print_properties(devicename, format, Source::Sysname)?;
				}
			};
			if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES) {
				print_sysname_attributes(devicename, Format::Listing)?;
			};
			if !matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES) && !matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES) {
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

fn print_properties<S: Into<String>>(name: S, format: Format, source: Source) -> io::Result<()> {
	let name = name.into();
	match format {
		Format::Listing => {
			for (name, value) in urdig::udev::get_properties(name, source)? {
				println!("{} : {}", name, value);
			}
			Ok(())
		}
	}
}

fn print_sysname_attributes<S: Into<String>>(name: S, format: Format) -> io::Result<()> {
	let name = name.into();
	match format {
		Format::Listing => {
			for (name, value) in urdig::udev::get_attributes_by_sysname(name)? {
				println!("{} : {:?}", name, value);
			}
			Ok(())
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

const CLAP_SUBCOMMAND_SYSNAME: &str = "sysname";
const CLAP_SUBCOMMAND_SYSNAME_DESCRIPTION: &str = "print options for specific device, which is called via its sysname. You should NOT use the devnode. E.g. for '/dev/sda' use --name=sda";

const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME: &str = "name";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_SHORT: &str = "n";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_LONG: &str = "name";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_DESCRIPTION: &str = "Specifies the 'device name', which could be used. Could be the sysname, the devnode or the syspath. Will be detected automatically.";

const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES: &str = "properties";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_SHORT: &str = "p";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_LONG: &str = "properties";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_DESCRIPTION: &str = "If set, all properties of the given device will be shown";

const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES: &str = "attributes";
const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_SHORT: &str = "a";
const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_LONG: &str = "attributes";
const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_DESCRIPTION: &str = "If set, all attributes of the given device will be shown";

//----------------------------------------------