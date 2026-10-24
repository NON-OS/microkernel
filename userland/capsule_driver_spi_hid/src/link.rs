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

//! The device's link, kept once setup has chosen it: spi0's port and what
//! the firmware says the device is run with. The input poll is the HID
//! driver's own, shared with HID over I2C, and reaches its frame read with
//! the driver's state alone; the link waits for it here.

use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

use nonos_spi_hid::Params;

use crate::acpi::Choice;

#[derive(Clone, Copy)]
pub struct Link {
    pub port: u32,
    pub choice: Choice,
}

/// Port, mode, clock, opcodes, input, output and body addresses, the
/// line's `_UID` (bit 16 says a line is known) and pin; a port of zero is
/// no link yet.
static WORDS: [AtomicU32; 9] = [const { AtomicU32::new(0) }; 9];
const KNOWN: u32 = 1 << 16;

pub fn set(l: &Link) {
    let (c, p) = (&l.choice, &l.choice.params);
    let (uid, pin) = c.line.map_or((0, 0), |(u, p)| (KNOWN | u32::from(u), u32::from(p)));
    let ops = u32::from(p.read_opcode) << 8 | u32::from(p.write_opcode);
    let words = [l.port, c.mode.into(), c.hz, ops, p.input_address, p.output_address];
    let words = words.into_iter().chain([c.body_address, uid, pin]);
    WORDS.iter().zip(words).for_each(|(w, v)| w.store(v, Relaxed));
}

/// The link, once set.
pub fn get() -> Option<Link> {
    let [port, mode, hz, ops, input, output, body, uid, pin] =
        WORDS.each_ref().map(|w| w.load(Relaxed));
    let params = Params {
        read_opcode: (ops >> 8) as u8,
        write_opcode: ops as u8,
        input_address: input,
        output_address: output,
    };
    let line = (uid & KNOWN != 0).then_some((uid as u16, pin as u16));
    let choice = Choice { mode: mode as u8, hz, params, body_address: body, line };
    (port != 0).then_some(Link { port, choice })
}
