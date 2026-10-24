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
//! The CCID class descriptor of a smart-card interface: type 21h, 54
//! bytes, after the interface descriptor (CCID 1.1, 5.1, as recalled,
//! unverified).

use nonos_usbnet::desc::{walk, INTERFACE};

const CCID_DESCRIPTOR: u8 = 0x21;

pub(super) fn class_descriptor(raw: &[u8], number: u8) -> Option<&[u8]> {
    let mut inside = false;
    walk(raw).find(|d| {
        if d[1] == INTERFACE && d.len() >= 4 {
            inside = d[2] == number && d[3] == 0;
        }
        inside && d[1] == CCID_DESCRIPTOR && d.len() >= 54
    })
}
