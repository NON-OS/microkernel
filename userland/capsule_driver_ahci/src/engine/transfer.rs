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

use super::cmd_header::CmdHeader;
use super::port::Port;
use super::prd_count::data_bytes;
use crate::constants::ata::{ATA_READ_DMA_EXT, ATA_WRITE_DMA_EXT};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Read or write `sectors` sectors at `lba`. The span is held to the served
/// capacity here, before any command is built, whatever the caller checked.
/// A read or a write is done only when the HBA moved every byte it asked
/// for: PRDBC counts both directions (AHCI 1.3.1, 4.2.2), so a write the
/// drive never took is an error, not Ok.
pub fn transfer(
    port: &mut Port,
    regs: Regs,
    lba: u64,
    sectors: u32,
    write: bool,
) -> AhciResult<()> {
    if !super::span::within(port.capacity_sectors, lba, sectors) {
        return Err(AhciError::OutOfRange);
    }
    let bytes = data_bytes(sectors).ok_or(AhciError::OutOfRange)?;
    // The HBA of a wedged port may still own the command table.
    if port.wedged {
        return Err(AhciError::Wedged);
    }
    let cmd = if write { ATA_WRITE_DMA_EXT } else { ATA_READ_DMA_EXT };
    super::build::build_slot0(port, cmd, lba, sectors, write)?;
    super::issue::run(port, regs)?;
    if moved(port) != bytes {
        return Err(AhciError::ShortTransfer);
    }
    Ok(())
}

/// The bytes the HBA moved for slot 0's command: PRDBC in its command
/// header (AHCI 1.3.1, 4.2.2), written by the HBA, so read volatile.
fn moved(port: &Port) -> u32 {
    let at = port.clb.user_va() + core::mem::offset_of!(CmdHeader, prdbc) as u64;
    // SAFETY: the command list is mapped for the port's life and slot 0's
    // header is its first 32 bytes; PRDBC is a dword at offset 4, aligned.
    unsafe { core::ptr::read_volatile(at as *const u32) }
}
