// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the iwlwifi firmware-load path.
//!
//! The FH (Flow Handler) firmware-into-device transfer is a fixed sequence of
//! MMIO register writes defined by the hardware spec. These proofs include the
//! real `load_sections` / `load_firmware_chunk` code and run it against a
//! modeled device (`MockMmio`) that records every write and reports the FH
//! transfer as complete. The proofs then assert the exact register sequence,
//! values, and addressing the driver programs, so the code is validated
//! against the documented spec without any hardware.
//!
//! This validates the driver's register sequence. It does not and cannot
//! assert that a real Intel chip responds correctly; that still needs silicon.

// The driver is no_std and uses `alloc`; link it so the included sources
// (the supplicant's Vec replies) resolve when proven on the host.
extern crate alloc;

#[path = "mounts_radio.rs"]
mod mounts_radio;
#[path = "mounts_setup.rs"]
mod mounts_setup;
#[path = "mounts_wire.rs"]
mod mounts_wire;
pub use mounts_radio::*;
pub use mounts_setup::*;
pub use mounts_wire::*;

#[cfg(test)]
#[path = "tests_gen3.rs"]
mod tests_gen3;
#[cfg(test)]
#[path = "tests_join.rs"]
mod tests_join;
#[cfg(test)]
#[path = "tests_setup.rs"]
mod tests_setup;
#[cfg(test)]
use tests_gen3::*;
#[cfg(test)]
use tests_join::*;
