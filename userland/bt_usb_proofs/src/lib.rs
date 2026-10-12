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

//! Proofs for the Bluetooth USB capsule. Every module is test-only: the
//! capsule's own files are compiled here unchanged, and so are the
//! simulators of the HCI, SMP and HOGP proofs, so each side of the air is
//! the one already proven against the specification.

extern crate alloc;

#[cfg(test)]
#[path = "../../capsule_driver_bt_usb/src/link/mod.rs"]
mod link;
#[cfg(test)]
#[path = "../../capsule_driver_bt_usb/src/hid/modifier.rs"]
mod modifier;
#[cfg(test)]
mod usb;

#[cfg(test)]
#[allow(unused)]
#[path = "../../nonos_bt_hci_proofs/src/sim/mod.rs"]
mod hci_sim;
#[cfg(test)]
#[allow(unused)]
#[path = "../../nonos_bt_hogp_proofs/src/sim/mod.rs"]
mod hogp_sim;
#[cfg(test)]
#[allow(unused)]
#[path = "../../nonos_bt_smp_proofs/src/sim/mod.rs"]
mod smp_sim;

#[cfg(test)]
mod air;
#[cfg(test)]
mod env;
#[cfg(test)]
mod fixture;

#[cfg(test)]
mod credit_tests;
#[cfg(test)]
mod early_tests;
#[cfg(test)]
mod find_tests;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod modifier_tests;
#[cfg(test)]
mod mouse_tests;
#[cfg(test)]
mod pick_tests;
#[cfg(test)]
mod refuse_tests;
#[cfg(test)]
mod signal_tests;
#[cfg(test)]
mod trust_tests;
#[cfg(test)]
mod usb_tests;
