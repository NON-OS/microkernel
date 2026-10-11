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

//! One request through slot 0, polled: the command descriptor, the UTRD,
//! the doorbell, and the doorbell bit seen dropped. A request the host never
//! finishes is cleared through UTRLCLR; a clear that does not take leaves
//! the slot latched as the host's (Ctx::wedged).

use super::layout::{CMD_AT, PRDT_AT, RSP_AT, RSP_READ, SLOT, UTRD_LEN};
use super::prdt::entry;
pub use super::req::Req;
use super::utrd::{ocs, utrd};
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::hci::regs::{IS, IS_UTRCS, UTRLCLR, UTRLDBR};
use crate::ufs::hci::wait::until;
use crate::ufs::time::CLEAR_MS;

/// The response UPIU's first RSP_READ bytes, once the host reports success.
pub fn issue<H: Hw>(hw: &H, c: &Ctx, r: &Req) -> Result<[u8; RSP_READ], UfsError> {
    if c.slot_held(hw) {
        return Err(UfsError::Host);
    }
    let l = &c.lists;
    let prdt = match r.data_len {
        0 => None,
        n => Some(entry(l.data.bus, n).ok_or(UfsError::Host)?),
    };
    let slot = SLOT as usize * UTRD_LEN;
    // Zeroed, so a stale response cannot pass for this request's.
    l.ucd.put(RSP_AT, &[0u8; RSP_READ]);
    l.ucd.put(CMD_AT, r.upiu);
    if let Some(e) = prdt {
        l.ucd.put(PRDT_AT, &e);
    }
    l.utrl.put(slot, &utrd(r.cmd_type, r.dir, l.ucd.bus, prdt.is_some() as u16));
    let bit = 1 << SLOT;
    hw.w32(UTRLDBR, bit);
    if !until(hw, r.ms, || hw.r32(UTRLDBR) & bit == 0) {
        // A 0 frees the slot; every 1 leaves its slot alone.
        hw.w32(UTRLCLR, !bit);
        if !until(hw, CLEAR_MS, || hw.r32(UTRLDBR) & bit == 0) {
            c.wedged.set(true);
        }
        return Err(UfsError::Timeout);
    }
    hw.w32(IS, IS_UTRCS);
    let mut desc = [0u8; UTRD_LEN];
    l.utrl.get(slot, &mut desc);
    match ocs(&desc) {
        0 => {}
        v => return Err(UfsError::Ocs(v)),
    }
    let mut rsp = [0u8; RSP_READ];
    l.ucd.get(RSP_AT, &mut rsp);
    Ok(rsp)
}
