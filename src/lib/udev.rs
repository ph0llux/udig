use std::collections::HashMap;
use std::io;

use crate as udig;
use crate::traits::*;

pub fn get_subsystems() -> io::Result<Vec<String>> {
	let mut subsystems = Vec::with_capacity(80);
	let mut enumerator = udev::Enumerator::new()?;
	for device in enumerator.scan_devices()? {
		let property = device.property_value(udig::PROPERTY_VALUE_SUBSYSTEM).to_io_result()?;
		if !subsystems.contains(&property.to_str().to_io_result()?.to_string()) {
			subsystems.push(property.to_str().to_io_result()?.to_string())
		}
	}
	Ok(subsystems)
}

pub fn get_properties_by_sysname<S: Into<String>>(sysname: S) -> io::Result<HashMap<String, String>> {
	let mut properties: HashMap<String, String> = HashMap::with_capacity(40);
	let mut enumerator = udev::Enumerator::new()?;
	enumerator.match_sysname(sysname.into())?;
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
