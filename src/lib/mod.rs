pub mod traits;
pub mod udev;

const SUBSYSTEM_BLOCKDEVICES: &str = "block";
const SUBSYSTEM_NETWORK: &str = "net";

const PROPERTY_VALUE_SUBSYSTEM: &str = "SUBSYSTEM";

const ERROR_VALUE_NONE: &str = "NONE";

//- Environment Vars
pub const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
