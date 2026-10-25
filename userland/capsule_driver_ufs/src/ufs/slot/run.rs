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

//! One request through the slot, polled. Its parts are written, the
//! doorbell rung, and the doorbell bit's fall is the end. A request the
//! host never ends is taken back through UTRLCLR; a host that will not let
//! it go keeps the slot for good (Ctx::wedged), so no later request writes
//! memory the old one may still use.

use super::descriptor::{descriptor, ocs};
use super::layout::{CMD_AT, PRDT_AT, RSP_AT, RSP_READ, SLOT};
use super::prdt::entry;
pub use super::req::Req;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::CLEAR_MS;
use crate::ufs::facts::hci::{IS, IS_UTRCS, UTRLCLR, UTRLDBR};
use crate::ufs::facts::utp::UTRD_LEN;
use crate::ufs::host::wait::until;

/// The response's first RSP_READ bytes, once the host reports success.
pub fn run<H: Hw>(hw: &H, c: &Ctx, r: &Req) -> Result<[u8; RSP_READ], UfsError> {
    if c.slot_held(hw) {
        return Err(UfsError::Host);
    }
    let l = &c.lists;
    let prdt = match r.data_len {
        0 => None,
        n => Some(entry(l.data.bus, n).ok_or(UfsError::Host)?),
    };
    let at = SLOT as usize * UTRD_LEN;
    // Zeroed, so a stale response cannot pass for this request's.
    l.ucd.put(RSP_AT, &[0u8; RSP_READ]);
    l.ucd.put(CMD_AT, r.upiu);
    if let Some(e) = &prdt {
        l.ucd.put(PRDT_AT, e);
    }
    l.utrl.put(at, &descriptor(r.cmd_type, r.dir, l.ucd.bus, prdt.is_some() as u16));
    let bit = 1 << SLOT;
    hw.w32(UTRLDBR, bit);
    if !until(hw, r.ms, || hw.r32(UTRLDBR) & bit == 0) {
        // UTRLCLR frees each slot written 0 and leaves each 1 alone.
        hw.w32(UTRLCLR, !bit);
        if !until(hw, CLEAR_MS, || hw.r32(UTRLDBR) & bit == 0) {
            c.wedged.set(true);
        }
        return Err(UfsError::Timeout);
    }
    hw.w32(IS, IS_UTRCS);
    let mut desc = [0u8; UTRD_LEN];
    l.utrl.get(at, &mut desc);
    match ocs(&desc) {
        0 => {}
        v => return Err(UfsError::Ocs(v)),
    }
    let mut rsp = [0u8; RSP_READ];
    l.ucd.get(RSP_AT, &mut rsp);
    Ok(rsp)
}
