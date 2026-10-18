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

use super::header::{HDR_LEN, RESP_HDR_LEN};
use crate::constants::{MAC_LEN, MAX_ETHERNET_FRAME};

pub const STATUS_LEN: usize = 4;

pub const MAC_ADDRESS_PAYLOAD_LEN: usize = MAC_LEN;
pub const LINK_STATUS_PAYLOAD_LEN: usize = 1;

pub const RX_PAYLOAD_PREFIX_LEN: usize = 4;
/// Frames one batch carries at most: most of the 64 slot ring, so the
/// device keeps buffers to fill while the batch is on its way.
pub const BATCH_MAX_FRAMES: usize = 44;
/// Frame bytes one batch carries at most, prefixes included.
pub const BATCH_MAX_BYTES: usize = 64 * 1024;

/// The longest request: a header and one frame to send.
pub const REQUEST_MAX: usize = HDR_LEN + MAX_ETHERNET_FRAME;
/// Where the frame starts in an OP_RX_PACKET reply: after the header, the
/// status and the frame's length.
pub const RX_FRAME_AT: usize = RESP_HDR_LEN + STATUS_LEN + RX_PAYLOAD_PREFIX_LEN;
/// The longest reply the serve loop's buffer carries: one received frame.
pub const REPLY_MAX: usize = RX_FRAME_AT + MAX_ETHERNET_FRAME;
