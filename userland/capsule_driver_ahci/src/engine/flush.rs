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
use super::cmd_table::CmdTable;
use super::fis::FisH2D;
use super::port::Port;
use crate::constants::ata::{ATA_DEV_LBA, ATA_FLUSH_EXT, FIS_H2D_LEN_DWORDS};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

pub fn flush(port: &mut Port, regs: Regs) -> AhciResult<()> {
    // The HBA of a wedged port may still own the command table.
    if port.wedged {
        return Err(AhciError::Wedged);
    }
    let table_va = port.ctba.user_va();
    let fis = FisH2D::command(ATA_FLUSH_EXT, 0, 0, ATA_DEV_LBA);
    let header = CmdHeader {
        flags: FIS_H2D_LEN_DWORDS,
        pm: 0,
        prdtl: 0,
        prdbc: 0,
        ctba_low: port.ctba.device_addr() as u32,
        ctba_high: (port.ctba.device_addr() >> 32) as u32,
        rsv: [0; 4],
    };
    // SAFETY: as in build_slot0, the table and list are mapped for the
    // port's life and the engine reads them only once PxCI is set.
    unsafe {
        core::ptr::write_bytes(table_va as *mut u8, 0, core::mem::size_of::<CmdTable>());
        core::ptr::write_volatile(table_va as *mut FisH2D, fis);
        core::ptr::write_volatile(port.clb.user_va() as *mut CmdHeader, header);
    }
    super::issue::run(port, regs)
}
