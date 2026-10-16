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

//! The host command queue of an AX210-family part (Linux v6.12 pcie/tx-gen2.c
//! `iwl_pcie_gen2_enqueue_hcmd`).
//!
//! A command is the 8-byte wide header (`iwl_cmd_header_wide`: opcode, group,
//! sequence, payload length, version) and its payload. It is handed to the
//! device by a 256-byte TFH descriptor (`iwl_tfh_tfd`: a buffer count and up
//! to 25 `[len u16][addr u64]` buffers): the first buffer is the command's
//! first 20 bytes copied into the slot's own small buffer, the second the
//! rest. The write pointer runs over 16 bits (`max_tfd_queue_size` 65536),
//! the descriptor slot is it modulo the 128-entry ring, and the doorbell is
//! `HBUS_TARG_WRPTR = write_ptr | queue << 16`. The sequence (`queue << 8 |
//! index`) is what the firmware's reply carries back. Queue 0 is the command
//! queue (`IWL_MVM_DQA_CMD_QUEUE`).

mod queue;
mod sizes;
mod write;

pub use queue::CmdQueue;
pub use sizes::FIRST_TB;
