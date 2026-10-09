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

/*
 * The release stick names its Qwen tier in a live plan, and the kernel
 * imported it through its own USB driver: a stick that driver did not bring
 * up had no model to offer. Here the plan's sector and every file it names
 * are read through the firmware's driver when the machine has eight times
 * their size in memory, and the kernel imports from that copy.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use uefi::prelude::*;
use uefi::proto::media::block::BlockIO;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::Handle;

use super::memory::machine_bytes;
use super::plan::{live_imports, PLAN_LBA};
use super::range::read_range;
use crate::handoff::types::{
    DiskMirror, MirrorExtent, DISK_MIRROR_LEN, DISK_MIRROR_MAGIC, MIRROR_EXTENTS,
};

/* The machine must hold this many times the files' size. */
const HEADROOM: u64 = 8;

/// The mirror record of the disk behind `handle`, as (physical address,
/// length); `None` when it has no live plan with files to import, or the
/// machine has too little memory to keep them.
pub fn disk_mirror(bs: &BootServices, handle: Handle) -> Option<(u64, u64)> {
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    /* SAFETY: GetProtocol leaves the disk bound to the driver that holds it
     * BY_DRIVER, and nothing here uninstalls the interface while it is held. */
    let blk =
        unsafe { bs.open_protocol::<BlockIO>(params, OpenProtocolAttributes::GetProtocol).ok()? };
    let block = u64::from(blk.media().block_size());
    if block == 0 || block > 4096 || 4096 % block != 0 {
        return None;
    }
    let capacity = (blk.media().last_block() + 1).checked_mul(block)? / 512;
    let plan = read_range(bs, &blk, block, PLAN_LBA, 1)?;
    let imports = live_imports(&plan.bytes()[..512], capacity)?;
    let need: u64 = imports.iter().map(|&(_, bytes)| bytes).sum();
    if need.saturating_mul(HEADROOM) > machine_bytes(bs) || imports.len() >= MIRROR_EXTENTS {
        return None;
    }
    let mut kept = Vec::with_capacity(imports.len() + 1);
    kept.push((PLAN_LBA, 1, plan));
    for &(at, bytes) in &imports {
        let sectors = bytes.div_ceil(512);
        kept.push((at, sectors, read_range(bs, &blk, block, at, sectors)?));
    }
    let mut record = Box::new(DiskMirror {
        magic: DISK_MIRROR_MAGIC,
        capacity,
        count: kept.len() as u64,
        extents: [MirrorExtent::default(); MIRROR_EXTENTS],
    });
    for (slot, (lba, sectors, pages)) in record.extents.iter_mut().zip(kept) {
        *slot = MirrorExtent { lba, sectors, phys: pages.keep() };
    }
    Some((Box::leak(record) as *const DiskMirror as u64, DISK_MIRROR_LEN as u64))
}
