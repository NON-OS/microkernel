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

//! The configuration records the walk reads, each already cut to its
//! bLength and long enough for the fields taken from it.

use alloc::vec::Vec;

use super::binding::HidBinding;
use super::types::{Endpoint, Interface, DT_REPORT};

/// An interface record of at least 9 bytes.
pub fn interface(rec: &[u8]) -> Interface {
    Interface { number: rec[2], class: rec[5], subclass: rec[6], protocol: rec[7], report_len: 0 }
}

/// The report descriptor length a HID class descriptor gives first; a
/// record cut short, or naming another descriptor first, gives none.
pub fn with_report_len(iface: Interface, rec: &[u8]) -> Interface {
    match rec {
        [_, _, _, _, _, _, DT_REPORT, lo, hi, ..] => {
            Interface { report_len: u16::from_le_bytes([*lo, *hi]), ..iface }
        }
        _ => iface,
    }
}

/// The binding an endpoint record of at least 7 bytes makes with the
/// interface it is in, if any.
pub fn maybe_push(out: &mut Vec<HidBinding>, iface: Option<Interface>, rec: &[u8]) {
    let Some(iface) = iface else { return };
    let raw = u16::from_le_bytes([rec[4], rec[5]]);
    // Bits 15:13 are reserved (USB 2.0 9.6.6, as recalled, unverified): no
    // report endpoint sets them. Bits 12:11 count transactions and are read
    // past, as the size is bits 10:0.
    if raw >> 13 != 0 {
        return;
    }
    let ep = Endpoint {
        address: rec[2],
        attributes: rec[3],
        max_packet_size: raw & 0x07ff,
        interval: rec[6],
    };
    if let Some(binding) = HidBinding::from_pair(iface, ep) {
        out.push(binding);
    }
}
