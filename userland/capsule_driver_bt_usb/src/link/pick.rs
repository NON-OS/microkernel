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

//! Which advertiser is a HID device to connect to. Pure, so it is proven
//! against the advertisements real keyboards and mice send.

use nonos_bt_wire::hci::reports::AdvReport;

/// ADV_IND and ADV_DIRECT_IND: the two report types a central may connect
/// in answer to (Vol 4, Part E, 7.7.65.2; Vol 6, Part B, 2.3.1).
const CONNECTABLE: [u8; 2] = [0x00, 0x01];
/// Incomplete and Complete List of 16-bit Service UUIDs, and Appearance
/// (Core Specification Supplement, Part A, 1.1 and 1.12).
const UUIDS_16: [u8; 2] = [0x02, 0x03];
const APPEARANCE: u8 = 0x19;
/// The HID Service (Assigned Numbers, 3.4.2).
const HID_SERVICE: u16 = 0x1812;
/// Appearance category 0x00F, Human Interface Device: values 0x03C0 to
/// 0x03FF (Assigned Numbers, 2.6).
const HID_APPEARANCE: core::ops::RangeInclusive<u16> = 0x03c0..=0x03ff;

/// A connectable advertiser that names the HID service or a HID
/// appearance. Advertising data is a run of (length, type, data)
/// structures (Vol 3, Part C, 11); a length of zero ends it early, and one
/// that runs past the end ends the walk without a match.
pub fn hid_advertiser(r: &AdvReport) -> bool {
    if !CONNECTABLE.contains(&r.event_type) {
        return false;
    }
    let mut rest = r.data;
    while let Some((&len, tail)) = rest.split_first() {
        let Some(ad) = tail.get(..usize::from(len)).filter(|_| len > 0) else { return false };
        let (kind, value) = (ad[0], &ad[1..]);
        let u16s = value.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]]));
        let hid = match kind {
            k if UUIDS_16.contains(&k) => u16s.clone().any(|u| u == HID_SERVICE),
            APPEARANCE => value.len() == 2 && u16s.clone().any(|a| HID_APPEARANCE.contains(&a)),
            _ => false,
        };
        if hid {
            return true;
        }
        rest = &tail[usize::from(len)..];
    }
    false
}
