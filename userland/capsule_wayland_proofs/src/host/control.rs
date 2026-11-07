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

//! The control socket: one command a line, acted on the first client.
//!
//!   type <text>             each character as the router sends it
//!   key [mod+]...<name>     one key: esc enter tab backspace delete up down
//!                           left right home end pgup pgdn f1..f12 or a char
//!   motion <x> <y>          the pointer to a point of the window, its frame
//!                           included, as the router gives it
//!   click <x> <y> [button]  moved there, pressed and let go (1 left)
//!   press|release <button>
//!   wheel <dy> [dx]         notches where the pointer is; positive is away
//!                           from the person
//!   display <w> <h>         the display's size from now on
//!   dump <path>             the window's pixels as a PPM
//!   focus <0|1>             the window manager takes the keyboard away or
//!                           gives it back, as when another window is chosen
//!   clip-set <text>         the desktop clipboard holds <text>
//!   clip                    the desktop clipboard, said on stderr
//!   state                   the scene, said on stderr

use std::cell::Cell;
use std::io::Read;
use std::os::unix::net::UnixStream;

use nonos_app_skeleton::input as keys;

use super::Client;
use crate::input::{self, Kind};

/// Read what has come on `s`, acting on each whole line. False once it closed.
pub fn read(s: &mut UnixStream, buf: &mut Vec<u8>, clients: &mut [Client]) -> bool {
    let mut chunk = [0u8; 4096];
    let open = match s.read(&mut chunk) {
        Ok(0) => false,
        Ok(n) => {
            buf.extend_from_slice(&chunk[..n]);
            true
        }
        Err(_) => true,
    };
    while let Some(end) = buf.iter().position(|b| *b == b'\n') {
        let line: Vec<u8> = buf.drain(..=end).collect();
        let before = nonos_libc::world::with(|w| w.inbox.len());
        act(String::from_utf8_lossy(&line).trim(), clients);
        // The input it posted goes in the first client's record.
        let posted: Vec<(u32, Vec<u8>)> =
            nonos_libc::world::with(|w| w.inbox.iter().skip(before).cloned().collect());
        if let Some(rec) = clients.first_mut().and_then(|c| c.rec.as_mut()) {
            posted.iter().for_each(|(by, f)| rec.put(&crate::record::input_chunk(*by, f)));
        }
    }
    open
}

thread_local! {
    /// Where the pointer was last put: the router sends it with a wheel step.
    static AT: Cell<(i32, i32)> = const { Cell::new((0, 0)) };
}

/// The window with the keyboard, which the scripted pointer is over, as the
/// router names the window under the pointer.
fn focused_window(clients: &[Client]) -> Option<u32> {
    let scene = &clients.first()?.session.guest.scene;
    let all = std::iter::once(&scene.win).chain(scene.behind.iter());
    all.filter(|w| w.focused).find_map(|w| w.window).or(scene.win.window)
}

fn act(line: &str, clients: &mut [Client]) {
    input::on_window(focused_window(clients));
    let mut w = line.split_whitespace();
    let num = |v: Option<&str>| v.and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
    match w.next() {
        Some("type") => line[4..].trim_start().chars().for_each(|c| input::tap(c as u32, 0)),
        Some("key") => key(w.next().unwrap_or("")),
        Some("motion") => {
            let at = (num(w.next()), num(w.next()));
            AT.with(|a| a.set(at));
            input::motion(at.0, at.1);
        }
        Some("click") => {
            let (x, y) = (num(w.next()), num(w.next()));
            AT.with(|a| a.set((x, y)));
            input::click(w.next().map_or(1, |b| num(Some(b)) as u32), (x, y));
        }
        Some(op @ ("press" | "release")) => {
            let kind = if op == "press" { Kind::ButtonDown } else { Kind::ButtonUp };
            input::post(kind, 0, num(w.next()) as u32, (0, 0), (0, 0));
        }
        Some("wheel") => {
            let (dy, dx) = (num(w.next()), num(w.next()));
            input::wheel(dx, dy, AT.with(Cell::get));
        }
        Some("display") => {
            let (wd, ht) = (num(w.next()) as u32, num(w.next()) as u32);
            nonos_app_skeleton::desk::with(|d| (d.width, d.height) = (wd, ht));
        }
        Some("dump") => match (clients.first(), w.next()) {
            (Some(c), Some(path)) => super::dump::dump(&c.session.guest, path),
            _ => eprintln!("[host] dump: no client or no path"),
        },
        Some("focus") => match clients.first().and_then(|c| c.session.guest.scene.win.window) {
            Some(window) => nonos_app_skeleton::desk::keys_notice(window, num(w.next()) == 1),
            None => eprintln!("[host] focus: no window"),
        },
        Some("clip-set") => {
            let text = line[8..].trim_start().as_bytes().to_vec();
            nonos_app_skeleton::desk::with(|d| d.clipboard = text);
        }
        Some("clip") => nonos_app_skeleton::desk::with(|d| {
            eprintln!("[host] clipboard: {:?}", String::from_utf8_lossy(&d.clipboard));
        }),
        Some("state") => clients.iter().for_each(|c| super::dump::state(&c.session.guest)),
        _ => eprintln!("[host] unknown command: {line}"),
    }
}

/// `ctrl+shift+a`: the modifiers held as the router's flags, the key last.
fn key(spec: &str) {
    let mut flags = 0u16;
    let mut parts: Vec<&str> = spec.split('+').collect();
    let name = parts.pop().unwrap_or("");
    for m in parts {
        flags |= match m {
            "shift" => keys::MOD_SHIFT,
            "ctrl" => keys::MOD_CTRL,
            "alt" => keys::MOD_ALT,
            "meta" | "super" => keys::MOD_META,
            "altgr" => keys::MOD_ALTGR,
            _ => 0,
        };
    }
    let code = match name {
        "esc" => keys::KEY_ESC,
        "enter" => keys::KEY_ENTER,
        "tab" => keys::KEY_TAB,
        "backspace" => keys::KEY_BACKSPACE,
        "delete" => keys::KEY_DELETE,
        "up" => keys::KEY_UP,
        "down" => keys::KEY_DOWN,
        "left" => keys::KEY_LEFT,
        "right" => keys::KEY_RIGHT,
        "home" => keys::KEY_HOME,
        "end" => keys::KEY_END,
        "pgup" => keys::KEY_PAGE_UP,
        "pgdn" => keys::KEY_PAGE_DOWN,
        "space" => b' ' as u32,
        f if f.starts_with('f') && f.len() > 1 => {
            keys::KEY_F1 + f[1..].parse::<u32>().unwrap_or(1).saturating_sub(1)
        }
        c => c.chars().next().map_or(0, |c| c as u32),
    };
    input::tap(code, flags);
}
