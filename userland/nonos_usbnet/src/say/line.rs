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

//! One log line, tagged `[usbnet <driver>]` so `log usbnet` shows every
//! USB network driver's steps together. No line carries a station address,
//! and the device's vendor and product only when its bind failed.

use super::fmt::Line;
use crate::station::Taken;

/// `[usbnet <tag>] <parts...>`, cut to fit, ending in a newline.
pub fn say(tag: &[u8], parts: &[&[u8]]) {
    let mut line = Line::new();
    line.put(b"[usbnet ");
    line.put(tag);
    line.put(b"] ");
    for p in parts {
        line.put(p);
    }
    line.send();
}

/// `[usbnet <tag>] port <n> [<vid>:<pid>]: <what> [errno <e>]`.
pub fn say_port(tag: &[u8], port: u8, ids: Option<(u16, u16)>, what: &[u8], errno: Option<i32>) {
    let mut line = Line::new();
    line.put(b"[usbnet ");
    line.put(tag);
    line.put(b"] port ");
    line.dec(port as i64);
    if let Some((vendor, product)) = ids {
        line.put(b" ");
        line.hex(vendor as u32, 4);
        line.put(b":");
        line.hex(product as u32, 4);
    }
    line.put(b": ");
    line.put(what);
    if let Some(e) = errno {
        line.put(b" errno ");
        line.dec(e as i64);
    }
    line.send();
}

/// `[usbnet <tag>] port <n> up, <how the drawn address was taken>`.
pub fn say_up(tag: &[u8], port: u8, taken: Taken) {
    let how: &[u8] = match taken {
        Taken::Programmed => b"drawn station address programmed",
        Taken::Promiscuous => b"drawn station address, device filter opened",
        Taken::Unconfirmed => b"drawn station address not confirmed: device filter would not open",
    };
    let mut line = Line::new();
    line.put(b"[usbnet ");
    line.put(tag);
    line.put(b"] port ");
    line.dec(port as i64);
    line.put(b" up, ");
    line.put(how);
    line.send();
}
