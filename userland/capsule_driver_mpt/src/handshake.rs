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

//! A message through the doorbell and its reply back through it, sixteen
//! bits at a time: how the IOC is told its facts and its configuration
//! before it has any queue.

use crate::constants::regs::{
    DOORBELL_ACTIVE, DOORBELL_DWORDS_SHIFT, DOORBELL_FUNCTION_SHIFT, FUNCTION_HANDSHAKE,
    HIS_DOORBELL_INTERRUPT, HIS_IOP_DOORBELL_STATUS, REG_DOORBELL, REG_HOST_INTERRUPT_STATUS,
};
use crate::constants::timing::DOORBELL_MS;
use crate::regs::Regs;
use crate::wait;

/// The IOC has something for the host on the doorbell.
fn doorbell_int<R: Regs>(r: &R) -> bool {
    wait::until(DOORBELL_MS, || r.r32(REG_HOST_INTERRUPT_STATUS) & HIS_DOORBELL_INTERRUPT != 0)
}

/// The IOC has taken what the host wrote to the doorbell.
fn doorbell_ack<R: Regs>(r: &R) -> bool {
    wait::until(DOORBELL_MS, || r.r32(REG_HOST_INTERRUPT_STATUS) & HIS_IOP_DOORBELL_STATUS == 0)
}

/// Send `req` and read its reply into `reply`, which keeps as much as it
/// holds; returns the reply's length in bytes as the IOC gave it.
pub fn handshake<R: Regs>(r: &R, req: &[u32], reply: &mut [u8]) -> Result<usize, &'static str> {
    if r.r32(REG_DOORBELL) & DOORBELL_ACTIVE != 0 {
        return Err("doorbell in use");
    }
    r.w32(REG_HOST_INTERRUPT_STATUS, 0);
    let start = (FUNCTION_HANDSHAKE as u32) << DOORBELL_FUNCTION_SHIFT
        | (req.len() as u32) << DOORBELL_DWORDS_SHIFT;
    r.w32(REG_DOORBELL, start);
    if !doorbell_int(r) {
        return Err("handshake not taken");
    }
    r.w32(REG_HOST_INTERRUPT_STATUS, 0);
    if !doorbell_ack(r) {
        return Err("handshake not taken");
    }
    for &d in req {
        r.w32(REG_DOORBELL, d);
        if !doorbell_ack(r) {
            return Err("handshake message not taken");
        }
    }
    // The reply's length, in dwords, is in its third byte: known once the
    // first two words are in.
    let mut words = 2usize;
    let mut n = 0usize;
    while n < words {
        if !doorbell_int(r) {
            return Err("handshake reply cut short");
        }
        let w = (r.r32(REG_DOORBELL) as u16).to_le_bytes();
        r.w32(REG_HOST_INTERRUPT_STATUS, 0);
        if let Some(slot) = reply.get_mut(2 * n..2 * n + 2) {
            slot.copy_from_slice(&w);
        }
        n += 1;
        if n == 2 {
            words = (w[0] as usize * 2).max(2);
        }
    }
    // The IOC marks the end of the reply with one more interrupt.
    if !doorbell_int(r) {
        return Err("handshake not ended");
    }
    r.w32(REG_HOST_INTERRUPT_STATUS, 0);
    Ok(words * 2)
}
