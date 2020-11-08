extern crate clap;
extern crate udev;

use clap::{App, Arg};
use std::io;
use urdig;
use urdig::traits::*;

fn main() -> io::Result<()> {
	let matches = App::new(urdig::CARGO_PKG_NAME)
		.version(urdig::CARGO_PKG_VERSION)
		.author(urdig::CARGO_PKG_AUTHORS)
		.about(urdig::CARGO_PKG_DESCRIPTION)
		.subcommand(
			App::new(CLAP_SUBCOMMAND_SUBSYSTEMS)
				.about(CLAP_SUBCOMMAND_SUBSYSTEMS_DESCRIPTION)
				.args(&[Arg::with_name(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_NAME)
					.short(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_SHORT)
					.long(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_LONG)
					.help(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_DESCRIPTION)
					.takes_value(false)
					.required(false)]),
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
			subsystems(SubsystemAction::ListAll)?
		}
		return Ok(());
	//SYSNAME
	} else if let Some(matches) = matches.subcommand_matches(CLAP_SUBCOMMAND_SYSNAME) {
		if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME) {
			let devicename = matches.value_of(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME).to_io_result()?;
			println!("{} {} {}", CLAP_SUBCOMMAND_SYSNAME, SEPARATOR_COLON, devicename);
			if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES) {
				print_sysname_properties(devicename, Format::Listing)?;
			};
			if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES) {
				print_sysname_attributes(devicename, Format::Listing)?;
			};
		}
		return Ok(());
	};
	Ok(())
}

enum SubsystemAction {
	ListAll,
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
		}
	}
}

fn print_sysname_properties<S: Into<String>>(sysname: S, format: Format) -> io::Result<()> {
	match format {
		Format::Listing => {
			for (name, value) in urdig::udev::get_properties_by_sysname(sysname.into())? {
				println!("{} : {}", name, value);
			}
			Ok(())
		}
	}
}

fn print_sysname_attributes<S: Into<String>>(sysname: S, format: Format) -> io::Result<()> {
	match format {
		Format::Listing => {
			for (name, value) in urdig::udev::get_attributes_by_sysname(sysname.into())? {
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

//----------------------------------------------

const CLAP_SUBCOMMAND_SYSNAME: &str = "sysname";
const CLAP_SUBCOMMAND_SYSNAME_DESCRIPTION: &str = "print options for specific device, which is called via its sysname.";

const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME: &str = "name";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_SHORT: &str = "n";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_LONG: &str = "name";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_DESCRIPTION: &str = "Specifies the 'device name', which could be used.";

const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES: &str = "properties";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_SHORT: &str = "p";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_LONG: &str = "properties";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PROPERTIES_DESCRIPTION: &str = "If set, all properties of the given device will be shown";

const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES: &str = "attributes";
const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_SHORT: &str = "a";
const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_LONG: &str = "attributes";
const CLAP_SUBCOMMAND_SYSNAME_ARG_ATTRIBUTES_DESCRIPTION: &str = "If set, all attributes of the given device will be shown";

//----------------------------------------------

const SEPARATOR_COLON: &str = ":";