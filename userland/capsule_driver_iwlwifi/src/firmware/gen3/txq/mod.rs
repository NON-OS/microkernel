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

//! One transmit queue of an AX210-family part (Linux v6.12 pcie/tx-gen2.c
//! `iwl_txq_gen2_tx`, `iwl_txq_gen2_build_tx`, `iwl_pcie_gen2_update_byte_tbl`,
//! `iwl_txq_reclaim`).
//!
//! The firmware owns the queue's number and scheduling (TVQM); the host owns
//! its memory: a ring of 256-byte TFH descriptors, a byte count table of
//! 1024 `iwl_gen3_bc_tbl_entry` words (`TFD_QUEUE_BC_SIZE_GEN3_AX210`), a
//! 64-byte first-buffer slot per descriptor, and the frames themselves. A
//! frame goes out as the device command `[iwl_cmd_header][iwl_tx_cmd_gen3]
//! [802.11 header]` then its body: the first buffer is that command's first
//! 20 bytes (`IWL_FIRST_TB_SIZE`) copied into the slot, the second the rest of
//! the command and header rounded up to four bytes, the third the body. The
//! command header is `TX_CMD`, group 0, and the sequence `(queue & 0x1F) << 8
//! | index`. The byte count entry is the frame's length with the descriptor's
//! fetch chunks (`DIV_ROUND_UP(2 + 10 * buffers, 64) - 1`) in bits 14-15, and
//! the doorbell is `HBUS_TARG_WRPTR = write_ptr | queue << 16`, the write
//! pointer running over 16 bits (`max_tfd_queue_size` 65536) and the
//! descriptor index being it modulo the ring.
//!
//! The firmware answers each frame with a TX response naming the queue and
//! the next index it has not finished (`ssn`); the slots before it are free
//! again. A response naming more slots than are in flight is ignored (Linux
//! logs it and reclaims nothing). At most [`TXQ_FRAMES`] frames are in flight,
//! each in its own buffer, so a buffer is never rewritten while the device
//! may still read it.

mod answered;
mod layout;
mod queue;
mod send;
mod write;

pub use layout::*;
pub use queue::TxQueue;
