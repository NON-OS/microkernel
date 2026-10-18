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

//! What a device that is not a stick is, for the log (`report`).

/// An interface descriptor's type, as descriptors/wire.rs names it.
const DESC_INTERFACE: u8 = 0x04;

/// The first interface's class, subclass and protocol in a configuration
/// descriptor, as `0x00CCSSPP`, or 0 when it has none.
pub fn first_interface(raw: &[u8]) -> u32 {
    let mut pos = 0;
    while pos + 2 <= raw.len() {
        let len = raw[pos] as usize;
        if len < 2 || pos + len > raw.len() {
            return 0;
        }
        if raw[pos + 1] == DESC_INTERFACE && len >= 9 {
            return u32::from_be_bytes([0, raw[pos + 5], raw[pos + 6], raw[pos + 7]]);
        }
        pos += len;
    }
    0
}
