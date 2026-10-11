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

//! The adapter's own commands through the command register, its hard reset,
//! and the bring-up that leaves it with its mailboxes: run at start, and
//! again to bring the adapter back after a request it never completed.

use core::ptr::write_bytes;

use crate::constants::layout::{MAILBOXES, MBO_OFF, SENSE_OFF};
use crate::constants::regs::{
    CMD_DISABLE_INTERRUPT, CMD_INIT_EXTENDED_MAILBOX, CMD_INQUIRE_CONFIGURATION, CMD_START_MAILBOX,
    CTL_HARD_RESET, CTL_INTERRUPT_RESET, INT_CMDC, REG_COMMAND, REG_CONTROL, REG_DATA_IN,
    REG_INTERRUPT, REG_STATUS, ST_CMDINV, ST_CPRBSY, ST_DACT, ST_DFAIL, ST_DIRRDY, ST_HARDY,
    ST_INREQ,
};
use crate::constants::timing::{COMMAND_BYTE_MS, RESET_MS};
use crate::port::Port;
use crate::wait;

/// The hard reset's diagnostics take hold before the status is read.
const RESET_HOLD_MS: u64 = 2;

/// One adapter command: the opcode and its parameters through the command
/// register, the reply bytes read back, then its completion awaited and the
/// interrupt it latched cleared.
pub fn command(p: &Port, op: u8, params: &[u8], reply: &mut [u8]) -> Result<(), &'static str> {
    let idle = || p.read8(REG_STATUS) & ST_CPRBSY == 0;
    for &b in core::iter::once(&op).chain(params) {
        if !wait::until(COMMAND_BYTE_MS, idle) {
            return Err("command register busy");
        }
        p.write8(REG_COMMAND, b);
    }
    for r in reply.iter_mut() {
        if !wait::until(COMMAND_BYTE_MS, || p.read8(REG_STATUS) & (ST_DIRRDY | ST_CMDINV) != 0) {
            return Err("no reply from the adapter");
        }
        if p.read8(REG_STATUS) & ST_CMDINV != 0 {
            break;
        }
        *r = p.read8(REG_DATA_IN);
    }
    let done = wait::until(COMMAND_BYTE_MS, || p.read8(REG_INTERRUPT) & INT_CMDC != 0);
    let invalid = p.read8(REG_STATUS) & ST_CMDINV != 0;
    p.write8(REG_CONTROL, CTL_INTERRUPT_RESET);
    match (invalid, done) {
        (true, _) => Err("command invalid"),
        (false, false) => Err("command not completed"),
        (false, true) => Ok(()),
    }
}

/// START MAILBOX COMMAND: look at the outgoing mailboxes. It completes no
/// command and latches nothing.
pub fn kick(p: &Port) -> bool {
    if !wait::until(COMMAND_BYTE_MS, || p.read8(REG_STATUS) & ST_CPRBSY == 0) {
        return false;
    }
    p.write8(REG_COMMAND, CMD_START_MAILBOX);
    true
}

/// A hard reset, its diagnostics, and the adapter ready and asking for
/// mailboxes.
pub fn reset(p: &Port) -> Result<(), &'static str> {
    p.write8(REG_CONTROL, CTL_HARD_RESET);
    wait::hold(RESET_HOLD_MS);
    let ready = wait::until(RESET_MS, || {
        let s = p.read8(REG_STATUS);
        s & ST_DACT == 0 && s & ST_HARDY != 0
    });
    let s = p.read8(REG_STATUS);
    if !ready {
        return Err("adapter not ready after reset");
    }
    if s & ST_DFAIL != 0 {
        return Err("adapter diagnostics failed");
    }
    if s & ST_INREQ == 0 {
        return Err("adapter did not ask for mailboxes");
    }
    Ok(())
}

/// Hard reset; the adapter's interrupt off (the driver polls, and an
/// adapter that does not know the command is left to Interrupt Disable);
/// its own SCSI ID, which is never a target; and the mailboxes at the
/// grant's start, which it must take. Returns the adapter's SCSI ID.
pub fn start(p: &Port, va: u64, dev: u64) -> Result<u8, &'static str> {
    reset(p)?;
    // SAFETY: the mailboxes and the CCB, the grant's first bytes, mapped
    // for the driver's life; the adapter was just reset and holds neither.
    unsafe { write_bytes(va as *mut u8, 0, SENSE_OFF) };
    let _ = command(p, CMD_DISABLE_INTERRUPT, &[0], &mut []);
    let mut cfg = [0u8; 3];
    command(p, CMD_INQUIRE_CONFIGURATION, &[], &mut cfg)?;
    let a = ((dev + MBO_OFF as u64) as u32).to_le_bytes();
    command(p, CMD_INIT_EXTENDED_MAILBOX, &[MAILBOXES, a[0], a[1], a[2], a[3]], &mut [])?;
    if p.read8(REG_STATUS) & ST_INREQ != 0 {
        return Err("mailboxes not taken");
    }
    Ok(cfg[2] & 0x0F)
}
