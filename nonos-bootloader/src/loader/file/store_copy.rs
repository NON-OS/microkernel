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
 * The package store, read whole through the firmware's own disk driver.
 *
 * The kernel reads the store through its USB, NVMe and SATA drivers, and a
 * machine whose stick or controller those drivers do not yet bring up got no
 * Linux tree, no apps and no Qwen catalogue, though the firmware had just
 * read the loader and kernel from that very disk. So the store is read here,
 * by the driver that booted the machine, into loader memory the kernel never
 * reclaims, and the kernel serves store reads from it. The disk the loader
 * came from is asked first; any other disk that carries a store after it,
 * unless the loader came from a USB stick: a stick's boot is never the
 * installed disk's, so it takes no other disk's store, and the kernel keeps
 * no other disk (block_device/other_disk.rs). Only what the table of
 * contents names is read, at most the 120 MiB below the disk plan.
 */

use uefi::prelude::*;
use uefi::proto::media::block::BlockIO;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::Handle;

use super::boot_device::boot_device;
use super::device_kind::BootDevice;
use super::own_disk::ours;
use super::pages::{Pages, PAGE};
use super::read_retry::read_all;

/* The store's first byte: LBA 256 of 512-byte sectors. */
const STORE_AT: u64 = 256 * 512;
/* The disk plan's first byte, LBA 245760: the store ends below it. */
const PLAN_AT: u64 = 245_760 * 512;
const MAGIC: &[u8; 8] = b"NONOSTR1";
const HEADER: usize = 32;
const TOC_ENTRY: usize = 128;
const MAX_ENTRIES: usize = 512;
/* Header and table of contents, rounded up to whole pages. */
const HEAD_PAGES: usize = (HEADER + TOC_ENTRY * MAX_ENTRIES).div_ceil(PAGE);

/// The store's bytes from its header on, as (physical base, length), in
/// loader-data pages, the disk it was read from, and whether that disk holds
/// this loader's volume; `None` when no disk the firmware sees carries one.
pub fn store_copy(bs: &BootServices) -> Option<((u64, u64), Handle, bool)> {
    let handles = bs.find_handles::<BlockIO>().ok()?;
    let ours = ours(bs, &handles);
    let stick = boot_device(bs) == BootDevice::UsbStick;
    let first = handles.iter().zip(&ours).filter(|(_, &o)| o);
    let rest = handles.iter().zip(&ours).filter(|(_, &o)| !o && !stick);
    first.chain(rest).find_map(|(&h, &o)| copy_from(bs, h).map(|copy| (copy, h, o)))
}

fn copy_from(bs: &BootServices, handle: Handle) -> Option<(u64, u64)> {
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    /* SAFETY: GetProtocol borrows the interface for the life of the scope
     * alone, and leaves the disk bound to the driver that holds it BY_DRIVER;
     * nothing here uninstalls it while it is held. */
    let blk =
        unsafe { bs.open_protocol::<BlockIO>(params, OpenProtocolAttributes::GetProtocol).ok()? };
    let media = blk.media();
    let block = media.block_size() as u64;
    if !media.is_media_present()
        || media.is_logical_partition()
        || block == 0
        || STORE_AT % block != 0
        || PAGE as u64 % block != 0
        || (media.last_block() + 1).saturating_mul(block) < PLAN_AT
    {
        return None;
    }
    let total = {
        let head = Pages::new(bs, HEAD_PAGES)?;
        let bytes = head.bytes();
        read_all(&blk, block, STORE_AT / block, bytes)?;
        span(bytes)?
    };
    let pages = (total as usize).div_ceil(PAGE);
    let store = Pages::new(bs, pages)?;
    let bytes = store.bytes();
    read_all(&blk, block, STORE_AT / block, bytes)?;
    if &bytes[..8] != MAGIC {
        return None;
    }
    Some((store.keep(), total))
}

/* The bytes the header's table of contents names, from the header on; `None`
 * for a header that is not a store's or that names a byte past the plan. */
fn span(head: &[u8]) -> Option<u64> {
    if &head[..8] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(head[12..16].try_into().ok()?) as usize;
    if count > MAX_ENTRIES {
        return None;
    }
    let mut end = (HEADER + TOC_ENTRY * MAX_ENTRIES) as u64;
    for i in 0..count {
        let at = HEADER + i * TOC_ENTRY + 96;
        let offset = u64::from_le_bytes(head[at..at + 8].try_into().ok()?);
        let len = u64::from_le_bytes(head[at + 8..at + 16].try_into().ok()?);
        let last = offset.checked_add(len)?;
        if offset < STORE_AT || last > PLAN_AT {
            return None;
        }
        end = end.max(last - STORE_AT);
    }
    Some(end)
}
