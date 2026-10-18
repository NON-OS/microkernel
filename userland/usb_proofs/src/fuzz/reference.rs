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

//! The binding rules written out a second time, from the USB and HID
//! specifications rather than from the driver, so every fuzzed walk is
//! checked against an independent statement of what it should pick.

use crate::descriptors::types::HidKind;
use crate::protocol::MAX_HID_BINDINGS;

pub type Binding = (HidKind, u8, u8, u16);

/// The binding rules written out a second time, from the USB and HID
/// specifications rather than from the driver, so every fuzzed walk is
/// checked against an independent statement of what it should pick.
pub fn reference(raw: &[u8]) -> Option<Vec<Binding>> {
    if raw.len() < 9 || raw[0] < 9 || raw[1] != 0x02 {
        return None;
    }
    let total = usize::from(u16::from_le_bytes([raw[2], raw[3]]));
    if total < 9 || total > raw.len() {
        return None;
    }
    let walk = &raw[..total];
    let (mut at, mut iface, mut out) = (9usize, None, Vec::new());
    while walk.len() - at >= 2 {
        let len = usize::from(walk[at]);
        if len < 2 || len > walk.len() - at {
            return None;
        }
        let rec = &walk[at..at + len];
        match rec[1] {
            0x04 => iface = (len >= 9).then(|| [rec[2], rec[5], rec[6], rec[7]]),
            0x05 if len >= 7 => out.extend(iface.and_then(|i| endpoint(i, rec))),
            _ => {}
        }
        if out.len() == MAX_HID_BINDINGS {
            break;
        }
        at += len;
    }
    Some(out)
}

fn endpoint([number, class, subclass, protocol]: [u8; 4], rec: &[u8]) -> Option<Binding> {
    let (address, attributes) = (rec[2], rec[3]);
    let raw = u16::from_le_bytes([rec[4], rec[5]]);
    let size = raw & 0x07ff;
    let (kind, least) = match (class, subclass, protocol) {
        (0x03, 1, 1) => (HidKind::Keyboard, 8),
        (0x03, 1, 2) => (HidKind::Mouse, 3),
        (0x03, 1, _) => return None,
        // Read through its report descriptor, which says how long its
        // reports are; until then a packet need only hold a byte.
        (0x03, _, _) => (HidKind::Report, 1),
        _ => return None,
    };
    let interrupt_in = address & 0x80 != 0 && address & 0x0f != 0 && attributes & 0x03 == 0x03;
    // bInterval 0 is no polling rate; bits 15:13 are reserved.
    let fits = rec[6] != 0 && raw >> 13 == 0 && (least..=1024).contains(&size);
    (interrupt_in && fits).then_some((kind, number, address, size))
}
