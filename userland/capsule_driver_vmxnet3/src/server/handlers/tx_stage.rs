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

//! A frame copied into its transmit slot, padded to the wire minimum.

use crate::constants::MIN_WIRE_FRAME;

/// Copy `body` into `slot`, zero-padded to the wire minimum, and return the
/// length to send. The pad is zeroed every time: a slot reused for a short
/// frame must not carry the tail of the frame it held before.
pub fn stage(slot: &mut [u8], body: &[u8]) -> usize {
    let len = body.len().max(MIN_WIRE_FRAME);
    slot[..body.len()].copy_from_slice(body);
    slot[body.len()..len].fill(0);
    len
}
