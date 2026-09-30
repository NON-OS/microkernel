/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi client Settings, first-boot setup and net_core share.
//!
//! `driver` finds whichever Wi-Fi driver service is registered and speaks the
//! control protocol both drivers answer: status, scan, join, link and leave.
//! `saved` keeps the networks a person chose to remember, sealed with
//! ChaCha20-Poly1305 under a key the TPM derives for this machine and boot
//! state, in a fixed-size record in the NONOS store. `network` and `wire` are
//! the scan result and its wire format.

#![no_std]

extern crate alloc;

mod driver;
mod network;
mod saved;
mod wipe;
mod wire;

pub use driver::{
    find, join_text, ConnectResult, Driver, DriverStage, Link, ScanOutcome, ScanStats, OP_STATUS,
    WIFI_HDR,
};
pub use network::{ScanNetwork, SSID_MAX};
pub use saved::{
    forget, forget_all, keeps_state, load, remember, sealing_ready, SavedError, SavedList,
    PASS_MAX, SLOTS,
};
pub use wipe::wipe;
pub use wire::parse_scan;
