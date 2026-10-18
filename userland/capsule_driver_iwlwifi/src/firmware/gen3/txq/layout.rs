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

//! A transmit queue's sizes, and where each part lies in its area.

use super::super::plan::{FIRST_TB_SLOT, TFD_SIZE};

/// Descriptors per queue: `max(IWL_MGMT_QUEUE_SIZE, min_txq_size)` for
/// AX210, and the size every TVQM queue here is asked for.
pub const TXQ_ENTRIES: usize = 128;
/// Frames in flight per queue.
pub const TXQ_FRAMES: usize = 16;
/// One frame's buffer: the command, header and body of the largest frame.
pub const TX_BUF: usize = 2048;
/// `TFD_QUEUE_BC_SIZE_GEN3_AX210` entries of two bytes.
pub const BC_TABLE: usize = 1024 * 2;
/// The device command header before the transmit command.
pub(super) const CMD_HDR: usize = 4;

// Offsets inside one queue's area.
pub(super) const TFDS: usize = 0;
pub(super) const BC: usize = TFDS + TXQ_ENTRIES * TFD_SIZE;
pub(super) const FIRST: usize = BC + ((BC_TABLE + 4095) & !4095);
pub(super) const BUFS: usize = FIRST + TXQ_ENTRIES * FIRST_TB_SLOT;
/// One queue's area, rounded to pages.
pub const TXQ_STRIDE: usize = (BUFS + TXQ_FRAMES * TX_BUF + 4095) & !4095;
/// The transmit region: the management queue's area, then the data queue's.
pub const TX_REGION: usize = 2 * TXQ_STRIDE;
