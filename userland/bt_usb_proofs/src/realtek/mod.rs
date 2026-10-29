// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! End to end on the HP's RTL8821C: the patch loaded on a modelled ROM,
//! bring-up, then a BLE keyboard and a BLE mouse over whole links, with
//! Settings' own Bluetooth panel reading every step from the capsule's
//! NBTC board as the capsule keeps it.

#[path = "../../../capsule_driver_bt_usb/src/vendor/realtek/firmware.rs"]
mod firmware;
/// Settings' Bluetooth panel, unchanged, so the proofs read the capsule's
/// NBTC with the very code the page reads it with.
#[path = "../../../capsule_settings/src/bluetooth/panel/mod.rs"]
pub mod panel;

mod board_desk;
mod board_env;
mod drive;
mod rom;
mod settings;

mod bring_up_tests;
mod keyboard_tests;
mod log_tests;
mod mouse_tests;
