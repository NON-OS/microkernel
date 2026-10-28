// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The Bluetooth USB capsule's files, unchanged, against the proven simulators.

#![cfg(test)]
extern crate alloc;

mod air;
mod battery_link_tests;
mod bond_keep_tests;
mod bond_tests;
mod credit_tests;
mod directed_proofs;
mod early_tests;
mod env;
mod env_desk;
mod env_link;
mod find_tests;
mod fixture;
mod fixture_link;
#[path = "../../nonos_bt_hci_proofs/src/sim/mod.rs"]
pub mod hci_sim;
#[path = "../../nonos_bt_hogp_proofs/src/sim/mod.rs"]
pub mod hogp_sim;
mod hold_tests;
mod intel;
mod interleave_sim;
mod interleave_tests;
mod liar;
#[path = "../../capsule_driver_bt_usb/src/link/mod.rs"]
mod link;
mod link_tests;
#[path = "../../capsule_driver_bt_usb/src/hid/modifier.rs"]
mod modifier;
mod modifier_tests;
mod mouse_tests;
#[path = "../../capsule_driver_bt_usb/src/nbtc/mod.rs"]
pub mod nbtc;
mod nbtc_proofs;
mod off_tests;
mod pair_tests;
mod pick_tests;
mod private_link_tests;
mod private_tests;
pub mod realtek;
mod reconnect_tests;
mod refuse_tests;
mod ring;
mod ring_refuse_tests;
mod ring_sim;
mod ring_sim_ops;
mod ring_tests;
mod ring_wire_tests;
pub mod settings;
mod signal_tests;
#[path = "../../nonos_bt_smp_proofs/src/sim/mod.rs"]
pub mod smp_sim;
mod trust_flags_tests;
mod trust_tests;
mod unauth_tests;
mod usb;
mod usb_tests;
