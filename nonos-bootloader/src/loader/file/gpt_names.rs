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
 * Whether a whole disk's GPT lists the partition this loader was read
 * from, by the GUID its volume's Hard Drive node names: the kernel finds
 * the boot disk the same way (block_device/gpt). The disk is opened with
 * GetProtocol, as store_copy.rs opens it, and only its table is read.
 */

use uefi::prelude::*;
use uefi::proto::media::block::BlockIO;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::Handle;

use super::gpt_entry::{entries, lists, ENTRY_BYTES_MAX};
use super::pages::{Pages, PAGE};
use super::read_retry::read_all;

pub(super) fn names(bs: &BootServices, disk: Handle, guid: &[u8; 16]) -> bool {
    let params = OpenProtocolParams { handle: disk, agent: bs.image_handle(), controller: None };
    /* SAFETY: GetProtocol borrows the interface for the life of the scope
     * alone, and nothing here uninstalls it while it is held. */
    let Ok(blk) =
        (unsafe { bs.open_protocol::<BlockIO>(params, OpenProtocolAttributes::GetProtocol) })
    else {
        return false;
    };
    let media = blk.media();
    let block = media.block_size() as u64;
    if !media.is_media_present() || media.is_logical_partition() || block == 0 {
        return false;
    }
    if PAGE as u64 % block != 0 {
        return false;
    }
    let Some(head) = Pages::new(bs, 1) else { return false };
    let header = &mut head.bytes()[..block as usize];
    if read_all(&blk, block, 1, header).is_none() {
        return false;
    }
    let Some((lba, count, size)) = entries(header) else { return false };
    let Some(table) = Pages::new(bs, ENTRY_BYTES_MAX.div_ceil(PAGE)) else { return false };
    let len = (count * size).div_ceil(block as usize) * block as usize;
    let bytes = &mut table.bytes()[..len];
    read_all(&blk, block, lba, bytes).is_some() && lists(&bytes[..count * size], size, guid)
}
