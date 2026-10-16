// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_driver_usb_hid/src/descriptors/application.rs"]
pub mod application;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/binding.rs"]
pub mod binding;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/config.rs"]
pub mod config;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/packet_size.rs"]
pub mod packet_size;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/pad/mod.rs"]
pub mod pad;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/pointer.rs"]
pub mod pointer;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/record.rs"]
mod record;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/types.rs"]
pub mod types;
// The parser returns Result<_, ()>, its own choice.
#[allow(clippy::result_unit_err)]
#[path = "../../../capsule_driver_usb_hid/src/descriptors/parse.rs"]
pub mod parse;
pub use pad::PadState;
pub use parse::hid_bindings;
pub use types::HidKind;
