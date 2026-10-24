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
//! Messages to the key, and a channel of our own asked for. Each report
//! goes out whole on the interrupt OUT endpoint; the INIT answer is waited
//! for on the IN one, while `wait` says there is time.

use nonos_usbnet::Bus;

use super::assemble::Assembler;
use super::frame::{packets, Cid, BROADCAST, CMD_INIT, PACKET};
use super::heard::Heard;
use super::init::{channel, Channel};

pub const E_IO: i32 = -5;
pub const E_INVAL: i32 = -22;
pub const E_TIMEDOUT: i32 = -110;

/// `data` as one `cmd` on `cid`, a report at a time.
pub fn send<B: Bus>(bus: &mut B, out: u8, cid: Cid, cmd: u8, data: &[u8]) -> Result<(), i32> {
    for p in packets(cid, cmd, data).ok_or(E_INVAL)? {
        if bus.interrupt_out(out, &p)? != PACKET {
            return Err(E_IO);
        }
    }
    Ok(())
}

/// A channel from the key, for `nonce`, on endpoints `dci` (IN, OUT).
pub fn allocate<B: Bus>(
    bus: &mut B,
    dci: (u8, u8),
    nonce: [u8; 8],
    mut wait: impl FnMut() -> bool,
) -> Result<Channel, i32> {
    send(bus, dci.1, BROADCAST, CMD_INIT, &nonce)?;
    let mut asm = Assembler::new(BROADCAST);
    let mut p = [0u8; PACKET];
    while wait() {
        let Some(n) = bus.interrupt_in_poll(dci.0, &mut p)? else { continue };
        if let Heard::Message(CMD_INIT, data) = asm.feed(&p[..n.min(PACKET)]) {
            if let Some(c) = channel(&data, nonce) {
                return Ok(c);
            }
        }
    }
    Err(E_TIMEDOUT)
}
