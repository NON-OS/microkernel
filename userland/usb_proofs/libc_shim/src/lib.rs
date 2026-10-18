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

//! What the included USB HID driver files take from `nonos_libc`: the input
//! event and its post, the clock and an idle, and the debug line. Everything
//! is per thread, so tests run side by side without seeing each other.

mod clock;
mod post;

pub use clock::{advance_ms, idled_ms, mk_idle_ms, mk_uptime_ms};
pub use post::{
    mk_input_event_post, take_posted, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_WHEEL,
};

/// A debug line goes nowhere on the host.
pub fn mk_debug(_buf: *const u8, len: usize) -> i64 {
    len as i64
}

/// A fresh thread: nothing posted, the clock at zero, nothing idled.
pub fn reset() {
    let _ = take_posted();
    clock::reset();
}
