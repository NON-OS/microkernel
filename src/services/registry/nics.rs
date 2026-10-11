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

//! The network cards' drivers, by the name each one's spawn registers.
//!
//! One table, because each name is read in several places that must agree:
//! the kernel holds a card to its stack (held_table.rs), raises its band
//! (install/priority.rs) and leaves it out of a profile with no network
//! (profile_refuse.rs); net.core and net.l2 look the cards up, the setup
//! wizard asks whether one is there, and capsule_attack tries to reach them.
//! A card left off one of those was reachable by any app, or never bound. The
//! capsules mount this file by path, so adding a card is a line here and its
//! spawn's name (held to this table in kernel_proofs).
//!
//! Only data: each reader takes what it needs, so no capsule carries code it
//! never calls.

/// What carries a card's frames, which decides who may drive it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Link {
    Wired,
    Wifi,
}

/// Every card, wired and Wi-Fi each in the order net.l2 and net.core try
/// them: QEMU's virtio-net first, then the real cards.
pub(crate) const NICS: &[(&str, Link)] = &[
    ("driver.virtio_net0", Link::Wired),
    ("driver.e1000_0", Link::Wired),
    ("driver.e1000e_0", Link::Wired),
    ("driver.rtl8169_0", Link::Wired),
    ("driver.rtl8139_0", Link::Wired),
    ("driver.igb_0", Link::Wired),
    ("driver.ixgbe_0", Link::Wired),
    ("driver.iwlwifi0", Link::Wifi),
    ("driver.rtl8821ce0", Link::Wifi),
];
