// NONOS Operating System (AGPL-3.0-or-later)

//! The background scanner, mounted file by file as src/serve/scanner/mod.rs
//! lays it out, with its proofs inside it.

#[path = "../../../../capsule_driver_rtl8821ce/src/serve/scanner/channels.rs"]
mod channels;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/scanner/drain.rs"]
mod drain;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/scanner/hop.rs"]
mod hop;
#[cfg(test)]
#[path = "../../scanner_tests/mod.rs"]
mod scanner_tests;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/scanner/state.rs"]
mod state;

pub(super) use channels::{SCAN_CHANNELS, SCAN_FRAME_MAX};
pub(super) use state::Scanner;
