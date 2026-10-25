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

//! A look at the memory modules' SPD EEPROMs once the host is up, so the
//! boot log shows the bus working: each of 0x50 to 0x57 that answers, and
//! its byte 2 (the DRAM type, JEDEC 21-C), read with the transfer the wire
//! serves. Reading an EEPROM moves only its address pointer.

use crate::ports::Ports;
use crate::say::Line;
use crate::setup::Driver;
use crate::xfer::{transfer, Kind, Xfer, BLOCK_MAX};

const SPD_FIRST: u8 = 0x50;
const SPD_LAST: u8 = 0x57;
const DRAM_TYPE: u8 = 2;

pub fn survey(d: &mut Driver) {
    let mut answered = 0u32;
    for addr in SPD_FIRST..=SPD_LAST {
        if let Some(kind) = read_type(&mut d.host, addr) {
            answered += 1;
            Line::new()
                .put(b"spd ")
                .hex(addr as u32, 2)
                .put(b" dram type ")
                .hex(kind as u32, 2)
                .send();
        }
    }
    let f = d.found;
    let mut line = Line::new();
    line.put(b"i801 ").hex(f.vendor as u32, 4).put(b":").hex(f.device as u32, 4);
    line.put(b" up, spd answered at ").hex(answered, 1).put(b" of 8 addresses").send();
}

fn read_type<P: Ports>(host: &mut P, addr: u8) -> Option<u8> {
    let x = Xfer {
        addr,
        read: true,
        kind: Kind::ByteData,
        cmd: DRAM_TYPE,
        len: 0,
        data: [0; BLOCK_MAX],
    };
    let mut out = [0u8; BLOCK_MAX];
    transfer(host, &x, &mut out).ok().map(|_| out[0])
}
