// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB network core. The shipped modules are
//! mounted at the same paths the drivers name (`nonos_usbnet::desc`, ...),
//! so a driver's proofs crate takes this crate as `nonos_usbnet`.

extern crate alloc;

#[path = "../../nonos_usbnet/src/bind.rs"]
pub mod bind;
#[path = "../../nonos_usbnet/src/bus.rs"]
pub mod bus;
#[path = "../../nonos_usbnet/src/desc/mod.rs"]
pub mod desc;
#[path = "../../nonos_usbnet/src/found.rs"]
pub mod found;
#[path = "../../nonos_usbnet/src/halt.rs"]
pub mod halt;
#[path = "../../nonos_usbnet/src/nic.rs"]
pub mod nic;
#[path = "../../nonos_usbnet/src/nnet/mod.rs"]
pub mod nnet;
#[path = "../../nonos_usbnet/src/run/mod.rs"]
pub mod run;
#[path = "../../nonos_usbnet/src/say/mod.rs"]
pub mod say;
#[path = "../../nonos_usbnet/src/scan/mod.rs"]
pub mod scan;
#[path = "../../nonos_usbnet/src/setup.rs"]
pub mod setup;
#[path = "../../nonos_usbnet/src/station.rs"]
pub mod station;
#[path = "../../nonos_usbnet/src/xhci/mod.rs"]
pub mod xhci;

/// A scripted device behind `Bus`, and the descriptors QEMU's usb-net shows.
pub mod mock;
pub mod qemu_usb_net;

pub use bind::Bind;
pub use bus::{Bus, Pipes};
pub use found::Found;
pub use nic::Nic;
pub use setup::Setup;
pub use station::{Station, Taken};

#[cfg(test)]
mod book_tests;
#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod desc_fuzz_tests;
#[cfg(test)]
mod desc_tests;
#[cfg(test)]
mod fake_nic;
#[cfg(test)]
mod fake_xhci;
#[cfg(test)]
mod found_tests;
#[cfg(test)]
mod manifest_tests;
#[cfg(test)]
mod nnet_tests;
#[cfg(test)]
mod say_tests;
#[cfg(test)]
mod scan_tests;
#[cfg(test)]
mod station_tests;
#[cfg(test)]
mod stats_tests;
#[cfg(test)]
mod xhci_client_tests;
#[cfg(test)]
mod xhci_count_tests;
