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

use crate::constants::{MAC_LEN, MAX_ETHERNET_FRAME};

pub const STATUS_LEN: usize = 4;
pub const MAX_TX_PAYLOAD_BYTES: u32 = MAX_ETHERNET_FRAME as u32;
pub const MAC_ADDRESS_PAYLOAD_LEN: usize = MAC_LEN;
pub const LINK_STATUS_PAYLOAD_LEN: usize = 1;
pub const RX_PAYLOAD_PREFIX_LEN: usize = 4;
pub const STATS_PAYLOAD_LEN: usize = 48;
/// Frames one batch carries at most: the 32 KiB receive ring holds about
/// twenty full frames, so the part keeps room to write while the batch is on
/// its way.
pub const BATCH_MAX_FRAMES: usize = 16;
/// Batch body bytes at most; net.core's reply buffer holds 64 KiB.
pub const BATCH_MAX_BYTES: usize = 64 * 1024;
