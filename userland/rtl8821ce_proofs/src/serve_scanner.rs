// NONOS Operating System (AGPL-3.0-or-later)
//! The background scanner's receive step, as serve/scanner holds it. Its
//! clock hop and log line (scanner/hop.rs) need the kernel and stay out.

#[path = "../../capsule_driver_rtl8821ce/src/serve/scanner/channels.rs"]
mod channels;
#[path = "../../capsule_driver_rtl8821ce/src/serve/scanner/drain.rs"]
mod drain;
#[path = "../../capsule_driver_rtl8821ce/src/serve/scanner/state.rs"]
mod state;

#[path = "scanner_tests/mod.rs"]
mod scanner_tests;
