// NONOS Operating System (AGPL-3.0-or-later)
//! The pure HID-over-I2C enumeration files; the firmware-facing
//! `enumerate` is left out, as it is for the extractor itself.
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/configs.rs"]
mod configs;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/dsm.rs"]
mod dsm;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/hids.rs"]
pub mod hids;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/names.rs"]
mod names;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/parse.rs"]
mod parse;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/resources.rs"]
mod resources;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/types.rs"]
mod types;
#[path = "../../../../../../../../src/arch/x86_64/acpi/devices/i2c/walk.rs"]
mod walk;

pub use parse::{parse_hid_devices, parse_platform_controllers, I2cHostName};
pub use types::{HidInterrupt, I2cHidDevice, I2cHidDeviceType};
