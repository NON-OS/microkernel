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

//! What a unit is called: INQUIRY's vendor and product, and its serial from
//! the Unit Serial Number VPD page (0x80), for the installer to show and to
//! take the erase-confirm word from. A unit without that page has no serial
//! to give, which is not a failure.

use super::cdb::cdb;
use super::codes::INQUIRY;
use super::exec::{exec, Data};
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::SCSI_MS;

/// Standard INQUIRY data up to the product revision.
const STD_LEN: usize = 36;
/// The VPD page header and up to 60 bytes of serial.
const VPD_LEN: usize = 64;
const VPD_SERIAL: u8 = 0x80;

/// Vendor (8) and product (16) as one 24-byte name, unpadded later.
pub fn name<H: Hw>(hw: &H, c: &Ctx, lun: u8) -> Result<[u8; 24], UfsError> {
    exec(hw, c, lun, query(false, 0, STD_LEN), Data::In(STD_LEN), SCSI_MS)?;
    let mut b = [0u8; STD_LEN];
    c.lists.data.get(0, &mut b);
    let mut out = [0u8; 24];
    out.copy_from_slice(&b[8..32]);
    Ok(out)
}

/// The serial and its length, or no serial when the page is not there.
pub fn serial<H: Hw>(hw: &H, c: &Ctx, lun: u8) -> ([u8; 60], usize) {
    let mut out = [0u8; 60];
    if exec(hw, c, lun, query(true, VPD_SERIAL, VPD_LEN), Data::In(VPD_LEN), SCSI_MS).is_err() {
        return (out, 0);
    }
    let mut b = [0u8; VPD_LEN];
    c.lists.data.get(0, &mut b);
    let n = core::cmp::min(b[3] as usize, out.len());
    out[..n].copy_from_slice(&b[4..4 + n]);
    (out, n)
}

fn query(vpd: bool, page: u8, alloc: usize) -> super::cdb::Cdb {
    cdb(6, |b| {
        b[0] = INQUIRY;
        b[1] = vpd as u8;
        b[2] = page;
        b[3..5].copy_from_slice(&(alloc as u16).to_be_bytes());
    })
}
