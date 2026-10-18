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

use crate::arch::fdt::find::uart::UartInfo;

use super::active::Active;
use super::line::line;
use super::{state, wait};

/// Make `info` the console and set its line up. Refused before anything
/// changes, so on an error the previous console is still the one in use.
pub fn init_uart(info: &UartInfo) -> Result<(), &'static str> {
    if info.base == 0 {
        return Err("the console UART has no address");
    }
    let line = line(info)?;
    state::select(info);
    if let Some(uart) = Active::current() {
        uart.init(line);
    }
    Ok(())
}

/// One raw byte. A transmitter with no room inside the wait budget drops
/// it and marks the port stalled, after which bytes are dropped without
/// waiting until it is seen to have room again: a console no one drains
/// must not turn every line of boot output into a pause.
pub fn write_byte(byte: u8) {
    let Some(uart) = Active::current() else {
        return;
    };
    let ready = if state::stalled() { uart.tx_ready() } else { wait::until(|| uart.tx_ready()) };
    state::set_stalled(!ready);
    if ready {
        uart.send(byte);
    }
}

/// One character, encoded, with no line-ending translation.
pub fn putc(c: char) {
    let mut buf = [0u8; 4];
    for &byte in c.encode_utf8(&mut buf).as_bytes() {
        write_byte(byte);
    }
}

/// Bytes as a terminal wants them, each `\n` preceded by a `\r`.
pub fn puts(s: &[u8]) {
    for &c in s {
        if c == b'\n' {
            write_byte(b'\r');
        }
        write_byte(c);
    }
}

pub fn getc() -> Option<u8> {
    Active::current()?.receive()
}
