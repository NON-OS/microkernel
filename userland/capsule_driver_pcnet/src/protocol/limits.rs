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

use crate::constants::MAC_LEN;

pub const STATUS_LEN: usize = 4;

pub const MAC_ADDRESS_PAYLOAD_LEN: usize = MAC_LEN;
pub const LINK_STATUS_PAYLOAD_LEN: usize = 1;
/// A received frame travels as a u32 length, then its bytes.
pub const RX_PAYLOAD_PREFIX_LEN: usize = 4;
/// Frames one batch carries at most: most of the 32-slot ring, so the part
/// keeps buffers to fill while the batch is on its way.
pub const BATCH_MAX_FRAMES: usize = 24;
/// Frame bytes one batch carries at most, prefixes included; net.core sizes
/// its reply buffer for this.
pub const BATCH_MAX_BYTES: usize = 64 * 1024;
