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

//! Proofs for the Bluetooth USB capsule: its files compiled unchanged, against
//! the HCI, SMP and HOGP proofs' simulators, each proven against the spec.

#![cfg(test)]

extern crate alloc;

#[allow(unused)]
#[path = "../../nonos_bt_hci_proofs/src/sim/mod.rs"]
mod hci_sim;
#[allow(unused)]
#[path = "../../nonos_bt_hogp_proofs/src/sim/mod.rs"]
mod hogp_sim;
#[path = "../../capsule_driver_bt_usb/src/link/mod.rs"]
mod link;
#[path = "../../capsule_driver_bt_usb/src/hid/modifier.rs"]
mod modifier;
#[allow(unused)]
#[path = "../../capsule_driver_bt_usb/src/nbtc/mod.rs"]
mod nbtc;
mod ring;
#[allow(unused)]
#[path = "../../nonos_bt_smp_proofs/src/sim/mod.rs"]
mod smp_sim;
mod usb;

mod air;
mod env;
mod env_desk;
mod env_link;
mod fixture;
mod fixture_link;
mod ring_sim;
mod ring_sim_ops;

mod battery_link_tests;
mod bond_keep_tests;
mod bond_tests;
mod credit_tests;
mod directed_proofs;
mod early_tests;
mod find_tests;
mod hold_tests;
mod link_tests;
mod modifier_tests;
mod mouse_tests;
mod nbtc_proofs;
mod pair_tests;
mod pick_tests;
mod private_link_tests;
mod private_tests;
mod reconnect_tests;
mod refuse_tests;
mod ring_refuse_tests;
mod ring_tests;
mod ring_wire_tests;
mod signal_tests;
mod trust_tests;
mod usb_tests;
