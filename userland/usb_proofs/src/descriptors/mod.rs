// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_driver_usb_hid/src/descriptors/binding.rs"]
pub mod binding;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/config.rs"]
pub mod config;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/packet_size.rs"]
pub mod packet_size;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/types.rs"]
pub mod types;
// The parser returns Result<_, ()>, its own choice.
#[allow(clippy::result_unit_err)]
#[path = "../../../capsule_driver_usb_hid/src/descriptors/parse.rs"]
pub mod parse;
pub use parse::hid_bindings;
pub use types::HidKind;
