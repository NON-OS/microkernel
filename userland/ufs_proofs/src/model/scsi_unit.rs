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

//! The commands a logical unit answers: UNIT ATTENTION first, then TEST UNIT
//! READY, READ CAPACITY and SYNCHRONIZE CACHE here, INQUIRY in
//! scsi_inquiry, READ and WRITE in scsi_io.

use super::rsp::sense;
use super::scsi::{put, Sense};
use super::sim::Model;
use crate::ufs::scsi::codes::*;

impl Model {
    pub(super) fn on_unit(
        &self,
        lun: u8,
        cdb: &[u8],
        data: Option<(u64, usize)>,
    ) -> Result<(), Sense> {
        let mut cfg = self.cfg.borrow_mut();
        let Some(lu) = cfg.lus.iter_mut().find(|l| l.lun == lun) else {
            return Err(sense(0x5, 0x25, 0));
        };
        if lu.unit_attention {
            lu.unit_attention = false;
            return Err(sense(KEY_UNIT_ATTENTION, 0x29, 0));
        }
        match cdb[0] {
            TEST_UNIT_READY if lu.not_ready > 0 => {
                lu.not_ready -= 1;
                Err(sense(KEY_NOT_READY, 0x04, 0x01))
            }
            TEST_UNIT_READY | SYNCHRONIZE_CACHE_10 => Ok(()),
            READ_CAPACITY_10 => {
                let last = u32::try_from(lu.blocks.wrapping_sub(1)).unwrap_or(u32::MAX);
                put(data, &[last.to_be_bytes(), lu.block_len.to_be_bytes()].concat());
                Ok(())
            }
            SERVICE_ACTION_IN_16 if cdb[1] == SA_READ_CAPACITY_16 => {
                let mut b = lu.blocks.wrapping_sub(1).to_be_bytes().to_vec();
                b.extend(lu.block_len.to_be_bytes());
                b.resize(32, 0);
                put(data, &b);
                Ok(())
            }
            INQUIRY => super::scsi_inquiry::inquiry(lu, cdb, data),
            READ_10 | READ_16 | WRITE_10 | WRITE_16 => super::scsi_io::rw(lu, cdb, data),
            _ => Err(sense(0x5, 0x20, 0)),
        }
    }
}
