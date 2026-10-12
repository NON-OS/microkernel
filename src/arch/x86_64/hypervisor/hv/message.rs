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

/*
 * A SynIC message slot (TLFS 11.10): 256 bytes, a 16-byte header (type,
 * payload size, flags with MessagePending in bit 0, reserved, origin) and
 * up to 240 bytes of payload. A slot whose type is zero is free.
 */

pub const MESSAGE_BYTES: usize = 256;
pub(super) const HEADER_BYTES: usize = 16;
pub const PAYLOAD_MAX: usize = MESSAGE_BYTES - HEADER_BYTES;

const FLAG_PENDING: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Slot {
    Free,
    /// A message: its type, its payload length (held to the slot), and
    /// whether the host has more queued behind it.
    Full {
        ty: u32,
        len: usize,
        pending: bool,
    },
}

pub(super) fn read(slot: &[u8; MESSAGE_BYTES]) -> Slot {
    let ty = u32::from_le_bytes([slot[0], slot[1], slot[2], slot[3]]);
    if ty == 0 {
        return Slot::Free;
    }
    let len = (slot[4] as usize).min(PAYLOAD_MAX);
    Slot::Full { ty, len, pending: slot[5] & FLAG_PENDING != 0 }
}
