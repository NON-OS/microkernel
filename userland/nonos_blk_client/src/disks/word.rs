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

//! The confirmation word a serial gives: its last four characters, lower
//! case. The NVMe specification makes a serial ASCII, but the bytes come
//! from the device. Slicing the last four bytes of one that ends in a
//! multi-byte character cut through it, which panics, and took the
//! installer down while it listed disks. A serial whose last four
//! characters are not printable ASCII gives no word, and the disk is
//! confirmed by its bus name instead, which [`bus_word`] makes.

use alloc::string::String;
use alloc::vec::Vec;

pub fn serial_word(serial: &str) -> Option<String> {
    let tail: Vec<char> = serial.chars().rev().take(4).collect();
    let typable = |c: &char| c.is_ascii() && !c.is_ascii_control();
    (tail.len() == 4 && tail.iter().all(typable))
        .then(|| tail.iter().rev().map(char::to_ascii_lowercase).collect())
}

/// A bus name, the instance after it past the first and the unit after a
/// dash past the first: "scsi", "scsi1", "ide-2", "scsi1-3".
pub fn bus_word(bus: &str, instance: u8, unit: u16) -> String {
    let mut word = String::from(bus);
    if instance > 0 {
        word.push_str(&alloc::format!("{instance}"));
    }
    if unit > 0 {
        word.push_str(&alloc::format!("-{unit}"));
    }
    word
}
