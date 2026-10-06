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

//! Where everything the gen3 boot hands the device lives in DMA memory.
//!
//! The kernel broker grants a network device at most 64 pages (256 KiB) per
//! DMA map, so the 1.6 MB firmware cannot sit in one region as the old 2 MiB
//! staging map assumed (the broker refused that map, so bring-up never got
//! past it). Instead: one control region holds the context info, peripheral
//! scratch and info, the receive status word and rings, the command ring and
//! its buffers, and the image loader; one region holds the receive buffers;
//! and the firmware sections are packed into as many regions as they need.
//! Every section gets its own page-aligned block, as Linux's one coherent
//! allocation per section does, and no block crosses a region.

use alloc::vec::Vec;

/// The largest region the broker grants a network device.
pub const GRANT_MAX: usize = 64 * PAGE;
pub const PAGE: usize = 4096;

/// Receive ring entries (`num_rbds` of Linux's non-HE configs; the ring size
/// is a parameter the firmware reads from the context info).
pub const RX_RING: usize = 512;
/// Receive buffers circulating in the ring.
pub const RX_BUFS: usize = 64;
/// Receive buffer size: 2 KiB, the AX210 family default (no RB size flag).
pub const RB_SIZE: usize = 2048;
/// Command ring entries: `max(IWL_CMD_QUEUE_SIZE, min_txq_size)` for AX210.
pub const CMD_RING: usize = 128;
pub const TFD_SIZE: usize = 256;
/// `IWL_FIRST_TB_SIZE_ALIGN`.
pub const FIRST_TB_SLOT: usize = 64;
/// One command in flight at a time; the largest is the 1948-byte scan request.
pub const CMD_BUF: usize = 4096;
pub const RX_FREE_DESC: usize = 16;
pub const RX_USED_DESC: usize = 32;

/// Offsets inside the control region.
pub const CTXT_INFO: usize = 0;
pub const PRPH_SCRATCH: usize = PAGE;
pub const PRPH_INFO: usize = 2 * PAGE;
pub const RB_STTS: usize = 3 * PAGE;
pub const RX_FREE: usize = 4 * PAGE;
pub const RX_USED: usize = RX_FREE + RX_RING * RX_FREE_DESC;
pub const CMD_TFDS: usize = RX_USED + RX_RING * RX_USED_DESC;
pub const FIRST_TBS: usize = CMD_TFDS + CMD_RING * TFD_SIZE;
pub const CMD_BUFFER: usize = FIRST_TBS + CMD_RING * FIRST_TB_SLOT;
pub const IML: usize = CMD_BUFFER + CMD_BUF;

/// The control region's size for an image loader of `iml_len` bytes and no
/// platform NVM, or `None` if it would not fit one grant.
#[cfg(test)]
pub fn control_len(iml_len: usize) -> Option<usize> {
    control_len_pnvm(iml_len, 0)
}

/// Where the platform NVM goes in the control region: the first page after
/// the image loader.
pub fn pnvm_off(iml_len: usize) -> Option<usize> {
    let end = IML.checked_add(iml_len)?;
    Some(end.checked_add(PAGE - 1)? & !(PAGE - 1))
}

/// The control region's size with `pnvm` bytes reserved after the image
/// loader for the platform NVM, or `None` if it would not fit one grant.
pub fn control_len_pnvm(iml_len: usize, pnvm: usize) -> Option<usize> {
    let len = pnvm_off(iml_len)?.checked_add(pnvm)?;
    (len <= GRANT_MAX).then_some(len)
}

/// The receive buffer region's size.
pub const RB_REGION: usize = RX_BUFS * RB_SIZE;

/// One firmware section's block: which firmware region, and where in it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Block {
    pub region: usize,
    pub off: usize,
    pub len: usize,
}

/// The firmware regions' sizes and every section's block, in the order the
/// lengths were given.
pub struct FwPlan {
    pub regions: Vec<usize>,
    pub blocks: Vec<Block>,
}

/// Pack sections of `lens` bytes, each page-aligned, first-fit into regions
/// of at most [`GRANT_MAX`]. `None` if a section is empty or larger than a
/// region.
pub fn plan_firmware(lens: &[usize]) -> Option<FwPlan> {
    let mut regions: Vec<usize> = Vec::new();
    let mut blocks = Vec::with_capacity(lens.len());
    for &len in lens {
        if len == 0 || len > GRANT_MAX {
            return None;
        }
        let fits = regions.last().is_some_and(|&used| used + len <= GRANT_MAX);
        if !fits {
            regions.push(0);
        }
        let region = regions.len() - 1;
        let off = regions[region];
        blocks.push(Block { region, off, len });
        regions[region] = (off + len + PAGE - 1) & !(PAGE - 1);
    }
    Some(FwPlan { regions, blocks })
}
