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
use super::cmd_table::{CmdTable, PRDT_OFFSET};
use super::fis::FisH2D;
use super::port::Port;
use super::prd_count::{data_bytes, prd_dbc};
use crate::constants::ata::{ATA_DEV_LBA, ATA_IDENTIFY, CMD_HEADER_WRITE, FIS_H2D_LEN_DWORDS};
use crate::error::{AhciError, AhciResult};

/// Write slot 0's command header, FIS and single PRD entry. The PRD count
/// comes from `prd_count`, so it never names more than the data buffer
/// holds; a count it refuses builds nothing.
pub(super) fn build_slot0(
    port: &Port,
    cmd: u8,
    lba: u64,
    sectors: u32,
    write: bool,
) -> AhciResult<()> {
    let bytes = data_bytes(sectors).ok_or(AhciError::OutOfRange)?;
    let dbc = prd_dbc(bytes).ok_or(AhciError::OutOfRange)?;
    let table_va = port.ctba.user_va();
    let data_phys = port.data.device_addr();
    let device = if cmd == ATA_IDENTIFY { 0 } else { ATA_DEV_LBA };
    let fis = FisH2D::command(cmd, lba, sectors, device);
    let mut flags = FIS_H2D_LEN_DWORDS;
    if write {
        flags |= CMD_HEADER_WRITE;
    }
    let header = CmdHeader {
        flags,
        pm: 0,
        prdtl: 1,
        prdbc: 0,
        ctba_low: port.ctba.device_addr() as u32,
        ctba_high: (port.ctba.device_addr() >> 32) as u32,
        rsv: [0; 4],
    };
    // SAFETY: the command table and list are mapped for the port's life,
    // a page each, and hold a CmdTable and a CmdHeader at their starts. The
    // engine fetches them only once PxCI is set, after this.
    unsafe {
        core::ptr::write_bytes(table_va as *mut u8, 0, core::mem::size_of::<CmdTable>());
        core::ptr::write_volatile(table_va as *mut FisH2D, fis);
        super::prdt_write::prdt_write(table_va + PRDT_OFFSET as u64, data_phys, dbc);
        core::ptr::write_volatile(port.clb.user_va() as *mut CmdHeader, header);
    }
    Ok(())
}
