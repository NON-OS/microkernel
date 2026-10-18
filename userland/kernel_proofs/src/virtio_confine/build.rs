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

//! A PCI config space as the capability walk reads it, for the K5 proofs.

/// A config space: the capabilities-present bit, the list head, and bytes.
pub(super) fn space(head: u8, caps: &[(u8, &[u8])]) -> impl Fn(u8) -> Option<u8> + Clone {
    let mut cfg = [0u8; 256];
    cfg[0x06] = 0x10;
    cfg[0x34] = head;
    for &(at, bytes) in caps {
        cfg[at as usize..at as usize + bytes.len()].copy_from_slice(bytes);
    }
    move |off| Some(cfg[off as usize])
}

pub(super) fn word(b: impl Fn(u8) -> Option<u8>) -> impl Fn(u8) -> Option<u32> {
    move |off| Some(u32::from_le_bytes([b(off)?, b(off + 1)?, b(off + 2)?, b(off + 3)?]))
}
