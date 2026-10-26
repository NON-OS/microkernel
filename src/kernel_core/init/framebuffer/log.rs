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

use super::hex::hex_u64;

pub(super) fn log_handoff_fb(ptr: u64, stride: u32, format: u32) {
    let (mut b1, mut b2, mut b3) = ([0u8; 16], [0u8; 16], [0u8; 16]);
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[FB] handoff ptr=0x").str(hex_u64(ptr, &mut b1));
    l.str(b" stride=0x").str(hex_u64(stride as u64, &mut b2));
    l.str(b" fmt=0x").str(hex_u64(format as u64, &mut b3)).end();
}
