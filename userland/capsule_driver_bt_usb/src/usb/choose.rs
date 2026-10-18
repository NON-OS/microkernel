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

//! Which of a device's configurations holds its Bluetooth interface, and
//! whether this driver can drive that interface. Pure, so the choice is
//! proven on the host against the descriptors real controllers report.

use alloc::vec::Vec;

use super::limit::read_limit;
use nonos_bt_hci::usb::{find, BtInterface};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Choice {
    /// bConfigurationValue, for SET_CONFIGURATION (USB 2.0, 9.6.3).
    pub value: u8,
    pub iface: BtInterface,
    /// The octets one look at the event endpoint arms.
    pub event_read: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Ours(Choice),
    /// No configuration has a Bluetooth interface: another driver's device.
    NotOurs,
    /// A Bluetooth controller this driver cannot drive, and why.
    Unusable(&'static str),
}

const CONFIG: u8 = 2;

/// The first configuration whose descriptor is whole and holds the
/// Bluetooth interface (Vol 4, Part B, 3).
pub fn choose(configs: &[Vec<u8>]) -> Verdict {
    for raw in configs {
        let &[len, CONFIG, _, _, _, value, ..] = raw.as_slice() else { continue };
        if len < 9 {
            continue;
        }
        let Ok(iface) = find(raw) else { continue };
        return match read_limit(iface.events.max_packet) {
            Some(event_read) => Verdict::Ours(Choice { value, iface, event_read }),
            None => Verdict::Unusable("event endpoint packet over 64 octets"),
        };
    }
    Verdict::NotOurs
}
