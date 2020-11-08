
use std::io;

use crate as urdig;

pub trait ToIOResult<T> {
	fn to_io_result(self) -> io::Result<T>;
}

impl<T, E: ToString> ToIOResult<T> for Result<T, E> {
	fn to_io_result(self) -> io::Result<T> {
		match self {
			Ok(x) => Ok(x),
			Err(err) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
		}
	}
}

impl<T> ToIOResult<T> for Option<T> {
	fn to_io_result(self) -> io::Result<T> {
		match self {
			Some(x) => Ok(x),
			None => Err(io::Error::new(io::ErrorKind::Other, urdig::ERROR_VALUE_NONE)),
		}
	}
}

pub trait OptionExtensions<T> {
	fn to_string_option(self) -> Option<String>;
}

impl<T: ToString> OptionExtensions<T> for Option<T> {
	fn to_string_option(self) -> Option<String> {
		match self {
			Some(x) => Some(x.to_string()),
			None => None,
		}
	}
}
