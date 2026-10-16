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

//! The CSW as real devices send it. Two things they do that the Bulk-Only
//! specification does not allow are taken as Linux's usb_stor_Bulk_transport
//! takes them. Some end a data phase with a zero-length packet it did not
//! need, which reaches the CSW read as an empty CSW; the CSW is read again.
//! Some skip the data phase of a command they fail and send the CSW at once,
//! which reaches the data read as 13 short bytes; those bytes are the CSW.
//! Before this the first failed every command with a transport error, and
//! the second waited out the bulk timeout for a CSW already sent, then reset
//! the device.

use super::recover::clear_halt;
use super::types::{Data, Disk};
use crate::bot::is_csw;
use crate::protocol::CSW_LEN;
use crate::xhci::{bulk_in, E_PIPE};

/// The CSW, read again after an empty packet and after a stall cleared.
pub(super) fn read_csw(disk: &Disk, csw: &mut [u8; CSW_LEN]) -> Result<usize, i32> {
    let mut got = bulk_in(disk.xhci, disk.slot, csw);
    if got == Ok(0) {
        got = bulk_in(disk.xhci, disk.slot, csw);
    }
    if got == Err(E_PIPE) {
        clear_halt(disk, true)?;
        got = bulk_in(disk.xhci, disk.slot, csw);
    }
    got
}

/// The CSW a device sent in place of a data-in phase it skipped: exactly 13
/// short bytes that carry the CSW signature. Linux checks the same two
/// things; the tag is checked with every other CSW's.
pub(super) fn skipped_data_phase(data: &Data, moved: usize) -> Option<[u8; CSW_LEN]> {
    let Data::In(buf) = data else { return None };
    if moved != CSW_LEN || buf.len() <= CSW_LEN || !is_csw(&buf[..CSW_LEN]) {
        return None;
    }
    buf[..CSW_LEN].try_into().ok()
}
