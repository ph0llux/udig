extern crate clap;
extern crate udev;

use clap::{App, Arg};
use std::io;
use udig;
use udig::traits::*;

fn main() -> io::Result<()> {
	let matches = App::new(udig::CARGO_PKG_NAME)
		.version(udig::CARGO_PKG_VERSION)
		.author(udig::CARGO_PKG_AUTHORS)
		.about(udig::CARGO_PKG_DESCRIPTION)
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
					Arg::with_name(CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_NAME)
						.short(CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_SHORT)
						.long(CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_LONG)
						.help(CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_DESCRIPTION)
						.takes_value(false)
						.required(false),
					Arg::with_name(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME)
						.short(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_SHORT)
						.long(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_LONG)
						.help(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_DESCRIPTION)
						.takes_value(true)
						.required(true),
				]),
		)
		.get_matches();
	//SUBSYSTEMS
	if let Some(matches) = matches.subcommand_matches(CLAP_SUBCOMMAND_SUBSYSTEMS) {
		if matches.is_present(CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_NAME) {
			subsystems(SubsystemAction::ListAll)?
		}
	} else if let Some(matches) = matches.subcommand_matches(CLAP_SUBCOMMAND_SYSNAME) {
		if matches.is_present(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME) {
			let devicename = matches.value_of(CLAP_SUBCOMMAND_SYSNAME_ARG_NAME).to_io_result()?;
			print_sysname_properties(devicename, Format::Listing)?;
		}
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
			for subsystem in udig::udev::get_subsystems()? {
				println!("{}", subsystem);
			}
			Ok(())
		}
	}
}

fn print_sysname_properties<S: Into<String>>(sysname: S, format: Format) -> io::Result<()> {
	match format {
		Format::Listing => {
			for (name, value) in udig::udev::get_properties_by_sysname(sysname.into())? {
				println!("{} : {}", name, value);
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
const CLAP_SUBCOMMAND_SUBSYSTEMS_ARG_LISTALL_DESCRIPTION: &str =
	"prints all available udev subsystems. For format option, use --format. By default, 'listing' format is used"; //TODO implement format
																											   //-------------------------------

const CLAP_SUBCOMMAND_SYSNAME: &str = "sysname";
const CLAP_SUBCOMMAND_SYSNAME_DESCRIPTION: &str = "print options for specific device, which is called via its sysname.";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_NAME: &str = "print-properties";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_SHORT: &str = "p";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_LONG: &str = "print-properties";
const CLAP_SUBCOMMAND_SYSNAME_ARG_PRINT_DESCRIPTION: &str =
	"prints all properties. For format option, use --format. By default, 'listing' format is used"; //TODO implement format

const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME: &str = "name";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_SHORT: &str = "n";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_LONG: &str = "name";
const CLAP_SUBCOMMAND_SYSNAME_ARG_NAME_DESCRIPTION: &str = "Specifies the 'device name', which could be used.";
