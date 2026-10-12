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

//! REPORT LUNS to the well-known REPORT LUNS LU: the logical units the
//! device has, as single-level LUNs (byte 0 zero, the LUN in byte 1).

use super::cdb::report_luns;
use super::codes::{WLUN_BIT, WLUN_REPORT_LUNS};
use super::exec::{exec, Data};
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::SCSI_MS;

/// Units this driver serves, as wire units 0 up: JESD220 allows 32 LUs and
/// devices ship with a handful.
pub const MAX_UNITS: usize = 8;
/// The list header and MAX_UNITS entries of 8 bytes.
const ALLOC: usize = 8 + 8 * MAX_UNITS;

pub fn report<H: Hw>(hw: &H, c: &Ctx) -> Result<([u8; MAX_UNITS], usize), UfsError> {
    exec(hw, c, WLUN_REPORT_LUNS, report_luns(ALLOC as u32), Data::In(ALLOC), SCSI_MS)?;
    let mut list = [0u8; ALLOC];
    c.lists.data.get(0, &mut list);
    Ok(parse(&list))
}

/// The LUNs, ascending, each once; a well-known or multi-level entry is
/// not a unit to serve.
pub fn parse(list: &[u8]) -> ([u8; MAX_UNITS], usize) {
    let mut out = [0u8; MAX_UNITS];
    let mut n = 0;
    let bytes = list.get(..4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]));
    let entries = list.get(8..).unwrap_or(&[]).chunks_exact(8).take(bytes as usize / 8);
    for e in entries {
        let lun = e[1];
        if e[0] != 0 || e[2..].iter().any(|&b| b != 0) || lun & WLUN_BIT != 0 {
            continue;
        }
        if n < MAX_UNITS && !out[..n].contains(&lun) {
            out[n] = lun;
            n += 1;
        }
    }
    out[..n].sort_unstable();
    (out, n)
}
