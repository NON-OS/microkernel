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

//! Open one window, draw it, and keep it up while answering the compositor.

use super::conn::Conn;
use super::ids::{BUF, COMP, POOL, REG, SHM, SURF, SYNC, TOP, XDG, XSURF};
use super::msg::{word, Msg};
use crate::sys::out;

const W: u32 = 480;
const H: u32 = 320;

pub fn run() -> bool {
    let Some(mut c) = Conn::connect(b"/run/wayland-0") else {
        out(b"[GUEST] window: no display socket\n");
        return false;
    };
    c.send(&Msg::new(1, 1).u32(REG).bytes());
    c.send(&Msg::new(1, 0).u32(SYNC).bytes());
    let names = super::window_globals::read(&mut c, REG, SYNC);
    let Some([comp, shm, xdg]) = names else {
        out(b"[GUEST] window: registry lacks compositor, shm or xdg_wm_base\n");
        return false;
    };
    let bind = |name, iface: &[u8], v, id| Msg::new(REG, 0).u32(name).string(iface).u32(v).u32(id);
    c.send(&bind(comp, b"wl_compositor", 4, COMP).bytes());
    c.send(&bind(shm, b"wl_shm", 1, SHM).bytes());
    c.send(&bind(xdg, b"xdg_wm_base", 2, XDG).bytes());
    c.send(&Msg::new(COMP, 0).u32(SURF).bytes());
    c.send(&Msg::new(XDG, 2).u32(XSURF).u32(SURF).bytes());
    c.send(&Msg::new(XSURF, 1).u32(TOP).bytes());
    c.send(&Msg::new(TOP, 2).string("Hello from Linux on NØNOS".as_bytes()).bytes());
    c.send(&Msg::new(TOP, 3).string(b"nonos.window").bytes());
    c.send(&Msg::new(SURF, 6).bytes());
    if !super::window_globals::configured(&mut c, XDG, XSURF) {
        out(b"[GUEST] window: never configured\n");
        return false;
    }
    let Some(fd) = super::pixels::pixels(W, H) else {
        out(b"[GUEST] window: no shared memory\n");
        return false;
    };
    let size = W * H * 4;
    c.send_fd(&Msg::new(SHM, 0).u32(POOL).u32(size).bytes(), fd);
    c.send(&Msg::new(POOL, 0).u32(BUF).u32(0).u32(W).u32(H).u32(W * 4).u32(1).bytes());
    c.send(&Msg::new(SURF, 1).u32(BUF).u32(0).u32(0).bytes());
    c.send(&Msg::new(SURF, 2).u32(0).u32(0).u32(W).u32(H).bytes());
    c.send(&Msg::new(SURF, 6).bytes());
    out(b"[GUEST] window: drawn and committed, 480x320\n");
    // Stay up: answer every ping so the compositor does not call it hung.
    while let Some((object, opcode, body)) = c.event() {
        if object == XDG && opcode == 0 {
            c.send(&Msg::new(XDG, 3).u32(word(&body, 0)).bytes());
        }
    }
    true
}
