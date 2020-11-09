pub mod traits;
pub mod udev;

//- separators
pub const SEPARATOR_COLON: &str = ":";
pub const SEPARATOR_SYSTEM_DIRECTORY: &str = "/";

pub const PROPERTY_VALUE_SUBSYSTEM: &str = "SUBSYSTEM";
pub const PROPERTY_VALUE_SYSNAME: &str = "SYSNAME";

//- Error messages and values
pub const ERROR_VALUE_NONE: &str = "NONE";
pub const ERROR_NO_PROPERTIES_AND_ATTRIBUTES: &str = "You could use -p or -a to print device properties or/and attributes.";

//- Environment Vars
pub const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

//- Paths in system
pub const DEV: &str = "/dev/";
pub const SYS: &str = "/sys/";