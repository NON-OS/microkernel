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

//! The two waits a window makes: for the globals, and for its first configure.

use super::conn::Conn;
use super::msg::{word, Msg};

/// The registry's compositor, shm and xdg_wm_base names, bound by interface
/// name as a real client does, collected until the sync callback fires.
pub fn read(c: &mut Conn, reg: u32, sync: u32) -> Option<[u32; 3]> {
    let mut found = [0u32; 3];
    while let Some((object, opcode, body)) = c.event() {
        if object == sync && opcode == 0 {
            break;
        }
        if object != reg || opcode != 0 {
            continue;
        }
        let name = word(&body, 0);
        let len = word(&body, 4) as usize;
        let iface = body.get(8..8 + len.saturating_sub(1)).unwrap_or(&[]);
        let slot =
            [&b"wl_compositor"[..], b"wl_shm", b"xdg_wm_base"].iter().position(|i| *i == iface);
        if let Some(i) = slot {
            found[i] = name;
        }
    }
    found.iter().all(|n| *n != 0).then_some(found)
}

/// Answer pings until the xdg_surface's first configure, then ack it.
pub fn configured(c: &mut Conn, xdg: u32, xsurf: u32) -> bool {
    while let Some((object, opcode, body)) = c.event() {
        if object == xdg && opcode == 0 {
            c.send(&Msg::new(xdg, 3).u32(word(&body, 0)).bytes());
        } else if object == xsurf && opcode == 0 {
            c.send(&Msg::new(xsurf, 4).u32(word(&body, 0)).bytes());
            return true;
        }
    }
    false
}
