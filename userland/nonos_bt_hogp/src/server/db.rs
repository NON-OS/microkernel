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
//! The attribute database this host serves to its peripheral. GAP asks
//! every LE device for a GAP service with Device Name and Appearance
//! (Vol 3, Part C, 12.1 and 12.2), and GATT for a GATT service (Vol 3,
//! Part G, 7). Every attribute here is readable and none is writable. The
//! database never changes, so the GATT service has no Service Changed
//! characteristic to announce changes with (Vol 3, Part G, 7.1).

use crate::uuids::{prop::READ, *};

pub struct Attr {
    pub handle: u16,
    pub kind: u16,
    pub value: &'static [u8],
}

/// What the peripheral reads as this host's name.
pub const NAME: &[u8] = b"NONOS";
/// Generic Computer, from the Appearance Values of the Bluetooth SIG's
/// Assigned Numbers; least significant octet first, like every ATT value.
pub const COMPUTER: [u8; 2] = [0x80, 0x00];

// A characteristic declaration's value: properties, value handle, UUID
// (Vol 3, Part G, 3.3.1).
pub const DB: [Attr; 6] = [
    Attr { handle: 1, kind: PRIMARY_SERVICE, value: &GAP_SERVICE.to_le_bytes() },
    Attr { handle: 2, kind: CHARACTERISTIC, value: &[READ, 3, 0, 0x00, 0x2a] },
    Attr { handle: 3, kind: DEVICE_NAME, value: NAME },
    Attr { handle: 4, kind: CHARACTERISTIC, value: &[READ, 5, 0, 0x01, 0x2a] },
    Attr { handle: 5, kind: APPEARANCE, value: &COMPUTER },
    Attr { handle: 6, kind: PRIMARY_SERVICE, value: &GATT_SERVICE.to_le_bytes() },
];

/// The attributes from `start` to `end`, both included.
pub fn in_range(start: u16, end: u16) -> impl Iterator<Item = &'static Attr> {
    DB.iter().filter(move |a| (start..=end).contains(&a.handle))
}

/// The attribute at `handle`.
pub fn at(handle: u16) -> Option<&'static Attr> {
    DB.iter().find(|a| a.handle == handle)
}

/// The last handle of the group `a` opens. A service runs up to the next
/// service, or to the end of the database (Vol 3, Part G, 3.1). Any other
/// attribute is its own group (Vol 3, Part F, 3.4.3.4).
pub fn group_end(a: &Attr) -> u16 {
    if a.kind != PRIMARY_SERVICE {
        return a.handle;
    }
    let next = DB.iter().find(|b| b.handle > a.handle && b.kind == PRIMARY_SERVICE);
    next.map_or(DB[DB.len() - 1].handle, |b| b.handle - 1)
}
