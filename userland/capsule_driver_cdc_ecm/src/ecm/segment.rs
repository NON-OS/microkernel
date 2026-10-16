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

//! wMaxSegmentSize (CDC ECM 1.2, 5.4, as recalled, unverified): the
//! largest frame the device carries, judged before it is configured.

use nonos_usbnet::nic::ETH_FRAME_MAX;

/// 0, or under a full frame, is refused: the stack hands down 1514-byte
/// frames, which such a device says it cannot carry. A size above the
/// receive buffer is clamped to the buffer, never used to size a read:
/// this driver turns on no larger frames, receives into a fixed buffer, and
/// lets go a transfer that fills it (receive.rs).
pub fn segment(max_segment: u16, buffer: usize) -> Result<usize, &'static str> {
    match max_segment as usize {
        n if n < ETH_FRAME_MAX => Err("segment size under one frame"),
        n => Ok(n.min(buffer)),
    }
}
