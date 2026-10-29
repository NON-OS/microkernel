// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Proofs for the RTL8821C patch loader: the capsule's own embedded files,
//! a modelled ROM that checks every fragment, and each way a load must
//! stop. Every module is test-only.

#![cfg(test)]

#[path = "../../capsule_driver_bt_usb/src/vendor/realtek/firmware.rs"]
mod firmware;
mod fixture;
mod model;

mod config_tests;
mod fragment_tests;
mod ids_tests;
mod load_tests;
mod patched_tests;
mod pin_tests;
mod refuse_tests;
mod select_tests;
mod truncated_tests;
