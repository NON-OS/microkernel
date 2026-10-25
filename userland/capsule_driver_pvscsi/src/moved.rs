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


//! What a completion's host status says of its request: a fault, or how much
//! of its data went unmoved. The completion's data length counts only on a
//! short transfer, which the emulated device marks BTSTAT_DATARUN and leaves
//! that length at zero (QEMU's hw/scsi/vmw_pvscsi.c, as constants/regs.rs
//! cites). A request that completed normally moved all it asked for: read as
//! a moved count, the zero turned every answer into an empty one, and no disk
//! behind the adapter was ever served. A short answer to anything but a read
//! or a write (inquiry, the LUN list, a VPD page) is taken whole: its buffer
//! is zeroed before it is sent and the answer carries its own length. A short
//! read or write with no length stays short, so it never passes for data.

use nonos_scsi::Fault;

use crate::constants::regs::{
    BT_ABORTQUEUE, BT_BUSFREE, BT_BUSRESET, BT_DATA_UNDERRUN, BT_HATIMEOUT,
    BT_LINKED_COMMAND_COMPLETED, BT_LINKED_COMMAND_COMPLETED_WITH_FLAG, BT_RECVRST, BT_SELTIMEO,
    BT_SENTRST, BT_SUCCESS,
};

/// A short transfer, over- or under-run, as the emulated device reports it.
pub const BT_DATARUN: u16 = 0x12;

/// The residual of a `len`-byte request that completed with `host`, its
/// completion reporting `moved` bytes, `rw` for a read or a write; or the
/// fault the status names.
pub fn completed(host: u16, len: usize, moved: u64, rw: bool) -> Result<u32, Fault> {
    match host {
        BT_SUCCESS | BT_LINKED_COMMAND_COMPLETED | BT_LINKED_COMMAND_COMPLETED_WITH_FLAG => Ok(0),
        BT_DATARUN if moved == 0 && !rw => Ok(0),
        BT_DATA_UNDERRUN | BT_DATARUN => Ok((len as u64).saturating_sub(moved) as u32),
        BT_SELTIMEO => Err(Fault::NoTarget),
        BT_BUSFREE | BT_SENTRST | BT_RECVRST | BT_BUSRESET | BT_ABORTQUEUE | BT_HATIMEOUT => {
            Err(Fault::Reset)
        }
        _ => Err(Fault::Failed),
    }
}
