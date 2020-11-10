use std::collections::HashMap;
use std::io;

use crate as urdig;
use crate::traits::*;

pub fn get_subsystems() -> io::Result<Vec<String>> {
	let mut subsystems = Vec::with_capacity(80);
	let mut enumerator = udev::Enumerator::new()?;
	for device in enumerator.scan_devices()? {
		let property = device.property_value(urdig::PROPERTY_VALUE_SUBSYSTEM).to_io_result()?;
		if !subsystems.contains(&property.to_str().to_io_result()?.to_string()) {
			subsystems.push(property.to_str().to_io_result()?.to_string())
		}
	}
	Ok(subsystems)
}

pub fn get_nodes_from_subsystem<S: Into<String>>(subsystem: S) -> io::Result<Vec<String>> {
	let mut devnodes: Vec<String> = Vec::new();
	let mut enumerator = udev::Enumerator::new()?;
	enumerator.match_subsystem(subsystem.into())?;
	for device in enumerator.scan_devices()? {
		match device.devnode() {
			Some(x) => devnodes.push(x.to_str().to_io_result()?.to_string()),
			None => devnodes.push(device.syspath().to_str().to_io_result()?.to_string())
		}
	}
	Ok(devnodes)
}

pub fn get_properties<S: Into<String>>(name: S, source: Source) -> io::Result<HashMap<String, String>> {
	fn by_sysname<S: Into<String>>(name: S) -> io::Result<HashMap<String, String>> {
		let mut properties: HashMap<String, String> = HashMap::with_capacity(40);
		let mut enumerator = udev::Enumerator::new()?;
		enumerator.match_sysname(name.into().trim())?;
		for device in enumerator.scan_devices()? {
			for property in device.properties() {
				properties.insert(
					property.name().to_str().to_io_result()?.to_string(),
					property.value().to_str().to_io_result()?.to_string(),
				);
			}
		}
		Ok(properties)
	}
	fn by_devnode<S: Into<String>>(name: S) -> io::Result<HashMap<String, String>> {
		let name = name.into();
		let mut split = name.trim().rsplit(urdig::SEPARATOR_SYSTEM_DIRECTORY);
		let name = split.next().to_io_result()?;
		let mut properties: HashMap<String, String> = HashMap::with_capacity(40);
		let mut enumerator = udev::Enumerator::new()?;
		enumerator.match_sysname(name)?;
		for device in enumerator.scan_devices()? {
			for property in device.properties() {
				properties.insert(
					property.name().to_str().to_io_result()?.to_string(),
					property.value().to_str().to_io_result()?.to_string(),
				);
			}
		}
		Ok(properties)
	}

	fn by_syspath<S: Into<String>>(name: S) -> io::Result<HashMap<String, String>> {
		let name = name.into();
		let mut split = name.trim().rsplit(urdig::SEPARATOR_SYSTEM_DIRECTORY);
		let name = split.next().to_io_result()?;
		let mut properties: HashMap<String, String> = HashMap::with_capacity(40);
		let mut enumerator = udev::Enumerator::new()?;
		enumerator.match_sysname(name)?;
		for device in enumerator.scan_devices()? {
			for property in device.properties() {
				properties.insert(
					property.name().to_str().to_io_result()?.to_string(),
					property.value().to_str().to_io_result()?.to_string(),
				);
			}
		}
		Ok(properties)
	}
	match source {
		Source::Sysname => by_sysname(name),
		Source::Devnode => by_devnode(name),
		Source::Syspath => by_syspath(name),
	}
}

pub fn get_attributes<S: Into<String>>(name: S, source: Source) -> io::Result<HashMap<String, String>> {
	fn by_sysname<S: Into<String>>(name: S) -> io::Result<HashMap<String, String>> {
		let mut attributes: HashMap<String, String> = HashMap::with_capacity(40);
		let mut enumerator = udev::Enumerator::new()?;
		enumerator.match_sysname(name.into().trim())?;
		for device in enumerator.scan_devices()? {
			for attribute in device.attributes() {
				let value = match attribute.value() {
						Some(x) => x.to_str().to_string(),
						None => urdig::ERROR_VALUE_NONE.to_string()
					};
				attributes.insert(
					attribute.name().to_str().to_io_result()?.to_string(),
					value,
				);
			}
		}
		Ok(attributes)
	}
	fn by_devnode<S: Into<String>>(name: S) -> io::Result<HashMap<String, String>> {
		let name = name.into();
		let mut split = name.trim().rsplit(urdig::SEPARATOR_SYSTEM_DIRECTORY);
		let name = split.next().to_io_result()?;
		let mut attributes: HashMap<String, String> = HashMap::with_capacity(40);
		let mut enumerator = udev::Enumerator::new()?;
		enumerator.match_sysname(name)?;
		for device in enumerator.scan_devices()? {
			for attribute in device.attributes() {
				let value = match attribute.value() {
						Some(x) => x.to_str().to_string(),
						None => urdig::ERROR_VALUE_NONE.to_string()
					};
				attributes.insert(
					attribute.name().to_str().to_io_result()?.to_string(),
					value,
				);
			}
		}
		Ok(attributes)
	}

	fn by_syspath<S: Into<String>>(name: S) -> io::Result<HashMap<String, String>> {
		let name = name.into();
		let mut split = name.trim().rsplit(urdig::SEPARATOR_SYSTEM_DIRECTORY);
		let name = split.next().to_io_result()?;
		let mut attributes: HashMap<String, String> = HashMap::with_capacity(40);
		let mut enumerator = udev::Enumerator::new()?;
		enumerator.match_sysname(name)?;
		for device in enumerator.scan_devices()? {
			for attribute in device.attributes() {
				let value = match attribute.value() {
						Some(x) => x.to_str().to_string(),
						None => urdig::ERROR_VALUE_NONE.to_string()
					};
				attributes.insert(
					attribute.name().to_str().to_io_result()?.to_string(),
					value,
				);
			}
		}
		Ok(attributes)
	}
	match source {
		Source::Sysname => by_sysname(name),
		Source::Devnode => by_devnode(name),
		Source::Syspath => by_syspath(name),
	}
}


pub enum Source {
	Sysname,
	Devnode,
	Syspath
}