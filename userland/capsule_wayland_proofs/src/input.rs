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

//! Input as the router delivers it: an NINP frame, sent from the router's
//! pid, put in the server's inbox for the next pump (input/pump.rs).

use std::cell::Cell;

use nonos_app_skeleton::desk::INPUT_ROUTER;
use nonos_libc::{DELIVERY_FLAG_WINDOW, DELIVERY_MAGIC, DELIVERY_VERSION};

thread_local! {
    static WINDOW: Cell<Option<u32>> = const { Cell::new(None) };
}

/// The window pointer events are routed to from now on, as the router names
/// the window under the pointer; None names none, as a key's frame does.
pub fn on_window(window: Option<u32>) {
    WINDOW.with(|w| w.set(window));
}

/// The router's kinds (app_skeleton input/decode.rs).
#[derive(Clone, Copy)]
pub enum Kind {
    KeyDown = 0,
    KeyUp = 1,
    PointerAbs = 3,
    Wheel = 4,
    ButtonDown = 5,
    ButtonUp = 6,
}

/// One routed event: its kind, modifier flags, code, place and deltas, in
/// the router's delivery frame; a pointer event names the window it is on.
pub fn post(kind: Kind, flags: u16, code: u32, (x, y): (i32, i32), (dx, dy): (i32, i32)) {
    let pointer = !matches!(kind, Kind::KeyDown | Kind::KeyUp);
    let window = WINDOW.with(Cell::get).filter(|_| pointer);
    let mut f = Vec::with_capacity(48);
    f.extend_from_slice(&DELIVERY_MAGIC.to_le_bytes());
    f.extend_from_slice(&DELIVERY_VERSION.to_le_bytes());
    let flags_word = if window.is_some() { DELIVERY_FLAG_WINDOW } else { 0 };
    f.extend_from_slice(&flags_word.to_le_bytes());
    f.extend_from_slice(&(kind as u16).to_le_bytes());
    f.extend_from_slice(&flags.to_le_bytes());
    f.extend_from_slice(&code.to_le_bytes());
    for v in [x, y, dx, dy] {
        f.extend_from_slice(&v.to_le_bytes());
    }
    f.extend_from_slice(&0u64.to_le_bytes());
    f.extend_from_slice(&window.unwrap_or(0).to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    nonos_libc::world::post(INPUT_ROUTER, f);
}

/// A key pressed and let go: a character, or a key's NONOS code.
pub fn tap(code: u32, flags: u16) {
    post(Kind::KeyDown, flags, code, (0, 0), (0, 0));
    post(Kind::KeyUp, flags, code, (0, 0), (0, 0));
}

pub fn motion(x: i32, y: i32) {
    post(Kind::PointerAbs, 0, 0, (x, y), (0, 0));
}

/// Button 1 is the left (pointer_codes.rs).
pub fn click(button: u32, (x, y): (i32, i32)) {
    motion(x, y);
    post(Kind::ButtonDown, 0, button, (x, y), (0, 0));
    post(Kind::ButtonUp, 0, button, (x, y), (0, 0));
}

pub fn wheel(dx: i32, dy: i32, (x, y): (i32, i32)) {
    post(Kind::Wheel, 0, 0, (x, y), (dx, dy));
}
