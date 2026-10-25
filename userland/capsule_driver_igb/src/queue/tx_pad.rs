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

//! A frame's bytes in its TX slot, padded to the shortest packet the MAC
//! sends. The stack may hand a bare 14-byte header; with TCTL.PSP the part
//! pads short frames to 60 bytes on the wire, but only from a 17-byte
//! buffer up (the the maintainers' review, R13, citing Intel's datasheets; the exact
//! clause is not checked here). So the slot holds the frame and zeros to
//! 17 bytes, and the descriptor names that length.

/// The shortest buffer length the transmit DMA takes.
pub const MIN_WIRE_LEN: usize = 17;

/// Copy `frame` to the start of `slot`, zero the slot up to the wire
/// length, and return that length. `None` when the slot cannot hold it.
pub fn fill(slot: &mut [u8], frame: &[u8]) -> Option<usize> {
    let wire = frame.len().max(MIN_WIRE_LEN);
    let dst = slot.get_mut(..wire)?;
    dst[..frame.len()].copy_from_slice(frame);
    dst[frame.len()..].fill(0);
    Some(wire)
}
