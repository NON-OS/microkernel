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

//! The access point's transmit queues: asking the firmware for one, giving
//! it back, and flushing what is still queued.
//!
//! `struct iwl_scd_queue_cfg_cmd` (fw/api/datapath.h),
//! TX_QUEUE_CFG_CMD_API_S_VER_3, 36 bytes: `operation` 0
//! (`IWL_SCD_QUEUE_ADD` 0, `_REMOVE` 1), then the union at 4. Adding
//! (TX_QUEUE_CFG_CMD_ADD_API_S_VER_1): `sta_mask` 4, `tid` 8 (a byte, three
//! reserved), `flags` 12, `cb_size` 16, `bc_dram_addr` 20, `tfdq_dram_addr`
//! 28. Removing: `sta_mask` 4, `tid` 8 (a word). Filled as Linux v6.12
//! pcie/tx-gen2.c `iwl_txq_dyn_alloc` and mvm/mld-sta.c
//! `iwl_mvm_mld_disable_txq` fill it: no flags, `cb_size` the ring's
//! `TFD_QUEUE_CB_SIZE` (log2 of the entries, less 3), the byte count table and
//! the descriptor ring's device addresses. Management frames take TID 15
//! (`IWL_MGMT_TID`), non-QoS data TID 0 (`IWL_TID_NON_QOS`), as
//! `iwl_mvm_tvqm_enable_txq` maps them.
//!
//! The reply is `struct iwl_tx_queue_cfg_rsp`, exactly 8 bytes (Linux
//! refuses any other length): `queue_number` 0, `flags` 2, `write_pointer` 4,
//! reserved 6. The queue's ring starts at that write pointer.
//!
//! `struct iwl_tx_path_flush_cmd`, TX_PATH_FLUSH_CMD_API_S_VER_2, 8 bytes:
//! `sta_id` 0, `tid_mask` 4 (every TID, 0xFFFF, as `iwl_mvm_flush_sta`
//! asks), reserved 6. Its reply lists what was flushed; the rings are reset
//! after a teardown, so it is not read.

use super::sta::AP_STA_ID;

pub const SCD_QUEUE_LEN: usize = 36;
pub const FLUSH_LEN: usize = 8;
const QUEUE_ADD: u32 = 0;
const QUEUE_REMOVE: u32 = 1;
const RSP_LEN: usize = 8;

/// `IWL_MGMT_TID` and `IWL_TID_NON_QOS`.
pub const MGMT_TID: u8 = 15;
pub const DATA_TID: u8 = 0;

/// `TFD_QUEUE_CB_SIZE` for a ring of `entries` (a power of two, at least 8).
pub fn cb_size(entries: usize) -> u32 {
    entries.trailing_zeros().saturating_sub(3)
}

/// Ask for a queue for `tid` of the stations in `sta_mask`, its ring of
/// `entries` descriptors at `tfds` and its byte count table at `bc`.
pub fn add_queue(sta_mask: u32, tid: u8, entries: usize, bc: u64, tfds: u64) -> [u8; SCD_QUEUE_LEN] {
    let mut c = [0u8; SCD_QUEUE_LEN];
    c[0..4].copy_from_slice(&QUEUE_ADD.to_le_bytes());
    c[4..8].copy_from_slice(&sta_mask.to_le_bytes());
    c[8] = tid;
    c[16..20].copy_from_slice(&cb_size(entries).to_le_bytes());
    c[20..28].copy_from_slice(&bc.to_le_bytes());
    c[28..36].copy_from_slice(&tfds.to_le_bytes());
    c
}

/// Give back the queue for `tid` of the stations in `sta_mask`.
pub fn remove_queue(sta_mask: u32, tid: u8) -> [u8; SCD_QUEUE_LEN] {
    let mut c = [0u8; SCD_QUEUE_LEN];
    c[0..4].copy_from_slice(&QUEUE_REMOVE.to_le_bytes());
    c[4..8].copy_from_slice(&sta_mask.to_le_bytes());
    c[8..12].copy_from_slice(&u32::from(tid).to_le_bytes());
    c
}

/// The queue the firmware gave: its number and the write pointer its ring
/// starts at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QueueGiven {
    pub queue: u16,
    pub write_ptr: u16,
}

/// Read the reply to an add, or `None` when it is not exactly the
/// structure's size.
pub fn parse_queue_reply(payload: &[u8]) -> Option<QueueGiven> {
    if payload.len() != RSP_LEN {
        return None;
    }
    Some(QueueGiven {
        queue: u16::from_le_bytes([payload[0], payload[1]]),
        write_ptr: u16::from_le_bytes([payload[4], payload[5]]),
    })
}

/// Flush every TID of the access point's station.
pub fn flush_sta() -> [u8; FLUSH_LEN] {
    let mut c = [0u8; FLUSH_LEN];
    c[0..4].copy_from_slice(&AP_STA_ID.to_le_bytes());
    c[4..6].copy_from_slice(&0xFFFFu16.to_le_bytes());
    c
}
