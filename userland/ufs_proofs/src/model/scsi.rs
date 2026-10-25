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

//! SCSI commands: REPORT LUNS on the well-known LU, the rest on a logical
//! unit, each answered GOOD or CHECK CONDITION with its sense. A data
//! buffer whose size is not what the command moves, or a UTRD direction
//! that is not the UPIU's R or W flag, is refused, so a driver that hands
//! the host the wrong PRDT or direction is caught.

use super::mem::{be32, write};
use super::rsp::{command, sense};
use super::sim::Model;
use crate::ufs::scsi::codes::*;

pub type Sense = Vec<u8>;

impl Model {
    pub(super) fn scsi(
        &self,
        u: &[u8; 32],
        dir: u32,
        data: Option<(u64, usize)>,
    ) -> Option<Vec<u8>> {
        if self.cfg.borrow().scsi_hang {
            return None;
        }
        let expected = be32(&u[12..16]) as usize;
        let want = match u[1] & 0x60 {
            0x40 => 2,
            0x20 => 1,
            _ => 0,
        };
        if data.map_or(0, |d| d.1) != expected || dir != want {
            return Some(command(CHECK_CONDITION, &sense(0x5, 0x24, 0)));
        }
        if self.mute.get() == Some(u[16]) {
            return Some(command(GOOD, &[]));
        }
        let r = if u[2] == WLUN_REPORT_LUNS {
            self.report_luns(data)
        } else {
            self.on_unit(u[2], &u[16..32], data)
        };
        Some(match r {
            Ok(()) => command(GOOD, &[]),
            Err(s) => command(CHECK_CONDITION, &s),
        })
    }
}

/// Copy `src` into the data buffer, as much as fits.
pub fn put(data: Option<(u64, usize)>, src: &[u8]) {
    if let Some((addr, len)) = data {
        let n = len.min(src.len());
        unsafe { write(addr, &src[..n]) };
    }
}
