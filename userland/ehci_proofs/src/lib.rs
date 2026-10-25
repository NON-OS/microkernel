// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the EHCI host. The shipped modules are mounted
//! at the paths the driver names them by (`crate::facts`, ...); only the
//! capsule's platform, setup and loop, which call the kernel, are left out.

#[path = "../../capsule_driver_ehci/src/devices/mod.rs"]
pub mod devices;
#[path = "../../capsule_driver_ehci/src/env/mod.rs"]
pub mod env;
#[path = "../../capsule_driver_ehci/src/facts/mod.rs"]
pub mod facts;
#[path = "../../capsule_driver_ehci/src/hc/mod.rs"]
pub mod hc;
#[path = "../../capsule_driver_ehci/src/sched/mod.rs"]
pub mod sched;
#[path = "../../capsule_driver_ehci/src/server/mod.rs"]
pub mod server;
#[path = "../../capsule_driver_ehci/src/wire/mod.rs"]
pub mod wire;
#[path = "../../capsule_driver_ehci/src/xfer/mod.rs"]
pub mod xfer;

/// A model controller and the devices on its ports.
pub mod model;
/// The broker's MMIO window rule, as the kernel builds it.
#[cfg(test)]
#[path = "../../../src/hardware/broker/mmio/window.rs"]
mod window;

#[cfg(test)]
mod bulk2_tests;
#[cfg(test)]
mod bulk_tests;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod ep0_fault_tests;
#[cfg(test)]
mod grant_tests;
#[cfg(test)]
mod hostile_qtd_tests;
#[cfg(test)]
mod hub_fixture;
#[cfg(test)]
#[path = "../../uhci_proofs/src/hub_lane.rs"]
mod hub_lane;
#[cfg(test)]
mod hub_refusal_tests;
#[cfg(test)]
#[path = "../../uhci_proofs/src/hub_seen.rs"]
mod hub_seen;
#[cfg(test)]
mod hub_tests;
#[cfg(test)]
#[path = "../../uhci_proofs/src/hub_wire.rs"]
mod hub_wire;
#[cfg(test)]
mod interrupt_tests;
#[cfg(test)]
mod many_tests;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod poll_share_tests;
#[cfg(test)]
mod poll_tests;
#[cfg(test)]
mod port_tests;
#[cfg(test)]
mod recover_tests;
#[cfg(test)]
mod start_tests;
#[cfg(test)]
mod wire_tests;
