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

//! Which configurations Ask holds: any with a HID interface, since a
//! keyboard need not offer the boot protocol and reading report
//! descriptors is more than the gate should do (a mouse waits too), or one
//! whose interfaces are of more than one class (USB 2.0 9.6.5, as
//! recalled, unverified). A CDC
//! Data interface belongs to the communications function beside it and is
//! not counted as a class of its own. A configuration naming no interface
//! is held too: nothing in it says what it is.

const INTERFACE: u8 = 0x04;
const CLASS_HID: u8 = 0x03;
const CLASS_CDC_DATA: u8 = 0x0A;

pub fn asks(config: &[u8]) -> bool {
    let (mut first, mut i) = (None, 0usize);
    while let Some(&len) = config.get(i) {
        let d = config.get(i..i + len as usize).filter(|_| len >= 2);
        let Some(d) = d else { return true };
        if d[1] == INTERFACE && d.len() >= 9 {
            let class = d[5];
            if class == CLASS_HID {
                return true;
            }
            if class != CLASS_CDC_DATA && *first.get_or_insert(class) != class {
                return true;
            }
        }
        i += len as usize;
    }
    first.is_none()
}
