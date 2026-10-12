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

//! One SCSI command to one logical unit, with its data in the data region:
//! the command UPIU's flags and expected length match the UTRD's direction
//! and the PRDT, and the response's status decides.

use super::cdb::Cdb;
use super::codes::{CHECK_CONDITION, GOOD};
use super::sense;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::upiu::{FLAG_READ, FLAG_WRITE, RESPONSE};
use crate::ufs::slot::descriptor::Dir;
use crate::ufs::slot::run::{run, Req};
use crate::ufs::upiu::request::command;
use crate::ufs::upiu::response::Rsp;

/// What moves: nothing, `n` bytes in from the device, or `n` bytes out.
#[derive(Clone, Copy)]
pub enum Data {
    None,
    In(usize),
    Out(usize),
}

pub fn exec<H: Hw>(
    hw: &H,
    c: &Ctx,
    lun: u8,
    cdb: Cdb,
    data: Data,
    ms: u64,
) -> Result<(), UfsError> {
    let (dir, flags, n) = match data {
        Data::None => (Dir::None, 0, 0),
        Data::In(n) => (Dir::FromDevice, FLAG_READ, n),
        Data::Out(n) => (Dir::ToDevice, FLAG_WRITE, n),
    };
    let upiu = command(lun, flags, n as u32, &cdb.0[..cdb.1]);
    let req = Req { cmd_type: c.scsi_type(), dir, upiu: &upiu, data_len: n, ms };
    let rsp = run(hw, c, &req)?;
    let r = Rsp(&rsp);
    if r.tt() != RESPONSE {
        return Err(UfsError::Upiu(r.tt()));
    }
    match r.status() {
        GOOD => Ok(()),
        CHECK_CONDITION => Err(sense::parse(r.sense())),
        s => Err(UfsError::Status(s)),
    }
}
