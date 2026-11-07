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

//! NONOS input events, as Wayland sees them.
//!
//! Routed, not drained: the personality subscribes to the input router like
//! any app and gets only what arrives while its surface has focus. It holds no
//! InputSource, and a frame from anyone but the router is not believed.

use nonos_app_skeleton::clients::input_router::subscribe;
use nonos_app_skeleton::discover::{from_router, lookup_port};
use nonos_app_skeleton::input::{parse_routed, InputEvent, InputKind};
use nonos_libc::{mk_ipc_recv_from, DELIVERY_LEN, DELIVERY_LEN_V1};

use super::took::{took, Took};
use crate::linux::guest::Guest;

/// Key down and up, absolute pointer, wheel, button down and up: one bit
/// per input kind (app_skeleton input/decode.rs), wheel being 4.
const KINDS: u32 = 0x7B;
const OWN_INBOX: u64 = 0;
const NOWAIT: u64 = 1;
/// Frames taken in one pass. A deeper backlog is drained on the next, which
/// may be a tick away; a held title bar's motions cost one move per pass
/// (`Held`), so a burst of them is taken whole.
const BATCH: usize = 128;

/// The pointer motion not yet delivered while the window is dragged by its
/// title bar. Each one delivered moves the window, two calls to the
/// compositor and the window manager it waits on, so a burst of N cost 2N
/// before the last position showed. Only the latest is kept, and it goes
/// before anything after it, so no button or key overtakes it.
type Held = Option<Routed>;

/// An event from the router, and the window manager window it names.
pub type Routed = (InputEvent, Option<u32>);

fn flush(guest: &mut Guest, held: &mut Held) {
    if let Some((event, window)) = held.take() {
        super::deliver::deliver(guest, &event, window);
    }
}

/// Whether the guest has a window open with the window manager.
fn windowed(guest: &Guest) -> bool {
    let scene = &guest.scene;
    scene.win.window.is_some() || scene.behind.iter().any(|w| w.window.is_some())
}

/// Whether `event` is a motion that only moves the dragged window.
fn drag_motion(guest: &Guest, event: &InputEvent) -> bool {
    guest.scene.win.frame.drag.is_some() && event.kind == InputKind::PointerAbs
}

pub fn pump(guest: &mut Guest) {
    super::super::clip::desk_read::drain(guest);
    // The inbox is always drained, so a window manager notice behind input
    // is never left to a full inbox: the keyboard given is said only once.
    let devices = guest.scene.pointer.is_some() || guest.scene.keyboard.is_some();
    // A window's NØNOS frame is the bridge's own: its title bar and controls
    // answer the pointer whether or not the client bound a seat.
    let wanted = devices || windowed(guest);
    if wanted && !guest.scene.subscribed {
        guest.scene.subscribed =
            lookup_port(b"input_router").is_some_and(|port| subscribe(port, 1, KINDS).is_ok());
    }
    let mut rx = [0u8; DELIVERY_LEN];
    let mut held: Held = None;
    for _ in 0..BATCH {
        let mut sender = 0u32;
        let n = next(&mut rx, &mut sender);
        match took(n, DELIVERY_LEN_V1) {
            Took::Done => break,
            // Not input: the desktop shell's focus-self (control.rs), else
            // the window manager's keyboard notice, if it is one.
            Took::Skip => {
                flush(guest, &mut held);
                let frame = &rx[..n as usize];
                if !super::control::took(guest, frame, sender) {
                    super::focus::notice(guest, frame, sender);
                }
                continue;
            }
            Took::Frame => {}
        }
        if !from_router(sender) {
            continue;
        }
        // A pointer event names the window it is over (input router v1).
        let frame = &rx[..(n as usize).min(rx.len())];
        if let Some((event, window)) = parse_routed(frame) {
            if !wanted {
                continue;
            }
            // A dragged window's motion moves the window, not the client,
            // so it goes whether or not the client is backed up.
            if drag_motion(guest, &event) {
                held = Some((event, window));
                continue;
            }
            flush(guest, &mut held);
            offer(guest, (event, window));
        }
    }
    flush(guest, &mut held);
    let backlogged = guest.display.backlogged();
    if let Some((event, window)) = guest.scene.motion_kept.release(backlogged) {
        super::deliver::deliver(guest, &event, window);
    }
}

/// The next frame: one SERVE_ONE's pump kept aside first (net/serve_one/aside.rs), then
/// the inbox, which only that pump reads while it serves.
fn next(rx: &mut [u8], sender: &mut u32) -> i64 {
    if let Some((n, from)) = crate::linux::net::serve_one::take_aside(rx) {
        *sender = from;
        return i64::try_from(n).unwrap_or(0);
    }
    if crate::linux::net::serve_one::serving() {
        return 0;
    }
    mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), NOWAIT, sender)
}

/// A client that has left a mebibyte unread is sent no pointer motion: its
/// queue grew for as long as the pointer moved. Keys, buttons and the wheel
/// still go, as each changes what the client holds. The latest motion not
/// sent goes before them, so a click lands where the pointer is
/// (latest.rs).
fn offer(guest: &mut Guest, routed: Routed) {
    let motion = routed.0.kind == InputKind::PointerAbs;
    let backlogged = guest.display.backlogged();
    for (event, window) in
        guest.scene.motion_kept.offer(routed, motion, backlogged).into_iter().flatten()
    {
        super::deliver::deliver(guest, &event, window);
    }
}
