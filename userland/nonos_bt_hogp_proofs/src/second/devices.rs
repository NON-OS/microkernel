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
//! Devices with more than one HID service.

use crate::sim::devices::{NOTIFY, READ, WWR};
use crate::sim::Db;

/// A service with only a boot keyboard input report, and no Report Map,
/// so it stays in boot mode.
pub fn boot_keyboard(db: Db) -> Db {
    db.service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
}

pub fn boot_mouse(db: Db) -> Db {
    db.service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char(READ | NOTIFY, 0x2a33, &[0; 3])
        .desc(0x2902, &[0, 0])
}

/// A service with no boot reports: `map` and one input Report with
/// Reference `reference`, and a Protocol Mode if `mode`.
pub fn report_only(db: Db, map: &[u8], reference: &[u8], mode: bool) -> Db {
    let db = db.service(0x1812);
    let db = if mode { db.char(READ | WWR, 0x2a4e, &[1]) } else { db };
    db.char(READ, 0x2a4b, map)
        .char(READ | NOTIFY, 0x2a4d, &[0; 8])
        .desc(0x2908, reference)
        .desc(0x2902, &[0, 0])
}

/// Each handle of `uuid`'s attributes, in database order.
pub fn handles(db: &Db, uuid: u16) -> Vec<u16> {
    let field = uuid.to_le_bytes();
    db.attrs.iter().filter(|a| a.kind == field).map(|a| a.handle).collect()
}

/// The Protocol Mode writes, in order: (handle, mode).
pub fn modes(writes: &[(u16, Vec<u8>, bool)]) -> Vec<(u16, Vec<u8>)> {
    writes.iter().filter(|w| w.2).map(|w| (w.0, w.1.clone())).collect()
}
