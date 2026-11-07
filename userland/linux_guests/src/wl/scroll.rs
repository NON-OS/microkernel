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

//! Keep a window scrolling: two buffers, each in a memfd of its own, the
//! one the compositor released redrawn eight rows further on and committed
//! at each frame callback, the band at the top left as it is. Every
//! REPLACE_EVERY frames both buffers and their pools are destroyed and
//! their memory let go, and two new ones made, as a client does when its
//! window changes size; each line it says names the machine's free memory,
//! so a run shows whether what a window lets go comes back.

use super::conn::Conn;
use super::ids::{COMP, REG, SHM, SURF, SYNC, TOP, XDG, XSURF};
use super::msg::{word, Msg};
use super::scroll_chain::Chain;
use crate::sys::out;

/// Each buffer 1,228,800 bytes, over the MiB the supervisor unmaps a span
/// in (capsule_linux guest/mem_unmap.rs), so a replaced buffer comes down
/// in two parts, as a larger client's does.
const W: u32 = 640;
const H: u32 = 480;
/// The band the scroll leaves alone.
const BAND: u32 = 40;
/// Frames between two lines on how long they took.
const EVERY: u32 = 120;
/// Frames between two replacements of the buffers.
const REPLACE_EVERY: u32 = 240;
/// The first new id after TOP: the pools, buffers and frame callbacks take
/// the ids after it in turn.
const FIRST_ID: u32 = 10;

pub fn run() -> bool {
    let Some(mut c) = Conn::connect(b"/run/wayland-0") else {
        out(b"[GUEST] scroll: no display socket\n");
        return false;
    };
    c.send(&Msg::new(1, 1).u32(REG).bytes());
    c.send(&Msg::new(1, 0).u32(SYNC).bytes());
    let Some([comp, shm, xdg]) = super::window_globals::read(&mut c, REG, SYNC) else {
        out(b"[GUEST] scroll: registry lacks compositor, shm or xdg_wm_base\n");
        return false;
    };
    let bind = |name, iface: &[u8], v, id| Msg::new(REG, 0).u32(name).string(iface).u32(v).u32(id);
    c.send(&bind(comp, b"wl_compositor", 4, COMP).bytes());
    c.send(&bind(shm, b"wl_shm", 1, SHM).bytes());
    c.send(&bind(xdg, b"xdg_wm_base", 2, XDG).bytes());
    c.send(&Msg::new(COMP, 0).u32(SURF).bytes());
    c.send(&Msg::new(XDG, 2).u32(XSURF).u32(SURF).bytes());
    c.send(&Msg::new(XSURF, 1).u32(TOP).bytes());
    c.send(&Msg::new(TOP, 2).string("Scrolling on NØNOS".as_bytes()).bytes());
    c.send(&Msg::new(TOP, 3).string(b"nonos.scroll").bytes());
    c.send(&Msg::new(SURF, 6).bytes());
    if !super::window_globals::configured(&mut c, XDG, XSURF) {
        out(b"[GUEST] scroll: never configured\n");
        return false;
    }
    let mut chain = Chain::new(FIRST_ID);
    if !chain.fill(&c, (W, H)) {
        out(b"[GUEST] scroll: no shared memory\n");
        return false;
    }
    let (mut frame, mut since) = (0u32, now_ms());
    out(format!("[GUEST] scroll: scrolling {W}x{H}\n").as_bytes());
    loop {
        if let Some(i) = chain.free_one() {
            let Some(slot) = chain.slots[i].as_mut() else { continue };
            slot.buf.draw(frame.wrapping_mul(8), BAND);
            slot.free = false;
            let id = slot.id;
            let callback = chain.take();
            c.send(&Msg::new(SURF, 1).u32(id).u32(0).u32(0).bytes());
            c.send(&Msg::new(SURF, 9).u32(0).u32(BAND).u32(W).u32(H - BAND).bytes());
            c.send(&Msg::new(SURF, 3).u32(callback).bytes());
            c.send(&Msg::new(SURF, 6).bytes());
            if !wait_frame(&mut c, callback, &mut chain) {
                return true;
            }
            frame = frame.wrapping_add(1);
            if frame % EVERY == 0 {
                let t = now_ms();
                say_frames(t.saturating_sub(since), chain.made);
                since = t;
            }
            if frame % REPLACE_EVERY == 0 {
                chain.drop_all(&c);
                if !chain.fill(&c, (W, H)) {
                    out(b"[GUEST] scroll: no shared memory for new buffers\n");
                    return false;
                }
            }
        } else if !answer_one(&mut c, &mut chain) {
            return true;
        }
    }
}

/// Events answered until frame callback `id` is done; false when the
/// display hung up.
fn wait_frame(c: &mut Conn, id: u32, chain: &mut Chain) -> bool {
    loop {
        let Some((object, opcode, body)) = c.event() else { return false };
        if object == id && opcode == 0 {
            return true;
        }
        answer(c, (object, opcode, &body), chain);
    }
}

fn answer_one(c: &mut Conn, chain: &mut Chain) -> bool {
    let Some((object, opcode, body)) = c.event() else { return false };
    answer(c, (object, opcode, &body), chain);
    true
}

/// A ping answered, a configure acked, a buffer given back marked free.
fn answer(c: &mut Conn, (object, opcode, body): (u32, u16, &[u8]), chain: &mut Chain) {
    if object == XDG && opcode == 0 {
        c.send(&Msg::new(XDG, 3).u32(word(body, 0)).bytes());
    } else if object == XSURF && opcode == 0 {
        c.send(&Msg::new(XSURF, 4).u32(word(body, 0)).bytes());
    } else if opcode == 0 {
        chain.released(object);
    }
}

fn now_ms() -> u64 {
    let mut ts = [0u64; 2];
    // CLOCK_MONOTONIC.
    crate::sys::call(228, [1, ts.as_mut_ptr() as u64, 0, 0, 0, 0]);
    ts[0].saturating_mul(1000).saturating_add(ts[1] / 1_000_000)
}

fn say_frames(ms: u64, made: u32) {
    let mut line = frame_line(ms);
    line.extend_from_slice(b", ");
    line.extend_from_slice(made.to_string().as_bytes());
    line.extend_from_slice(b" buffers made");
    if let Some(kb) = mem_free_kb() {
        line.extend_from_slice(b", ");
        line.extend_from_slice((kb / 1024).to_string().as_bytes());
        line.extend_from_slice(b" MiB free");
    }
    line.push(b'\n');
    out(&line);
}

fn frame_line(ms: u64) -> Vec<u8> {
    let mut line = b"[GUEST] scroll: 120 frames in ".to_vec();
    line.extend_from_slice(ms.to_string().as_bytes());
    line.extend_from_slice(b" ms, ");
    line.extend_from_slice((ms / u64::from(EVERY)).to_string().as_bytes());
    line.extend_from_slice(b" ms a frame");
    line
}

/// MemFree in /proc/meminfo: the machine's free memory, in KiB.
fn mem_free_kb() -> Option<u64> {
    let text = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = text.lines().find(|l| l.starts_with("MemFree:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}
