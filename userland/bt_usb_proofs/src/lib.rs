// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
