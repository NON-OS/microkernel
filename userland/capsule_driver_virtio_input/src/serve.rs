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

//! The poll loop: every device's used ring drained, each event taken into
//! its device's report or the keyboard, each buffer offered again, then a
//! short sleep when no device had anything.

use nonos_libc::mk_idle_ms;

use crate::constants::queue::{EVENT_BYTES, POLL_MS};
use crate::events::{post_pointer, Raw, Step};
use crate::keyboard::Keyboard;
use crate::setup::Input;

pub fn run(inputs: &mut [Input], kb: &mut Keyboard) -> ! {
    loop {
        let mut busy = false;
        for input in inputs.iter_mut() {
            busy |= pump(input, kb);
        }
        if !busy {
            let _ = mk_idle_ms(POLL_MS);
        }
    }
}

/// One pass over a device's used ring, bounded by its size. Returns whether
/// anything came back.
pub fn pump(input: &mut Input, kb: &mut Keyboard) -> bool {
    let mut seen = 0u16;
    while seen < input.ring.size {
        let Some(used) = input.ring.take() else { break };
        seen += 1;
        // An id past the ring names no buffer this driver offered: nothing
        // to read and nothing to give back.
        let Ok(id) = u16::try_from(used.id) else { continue };
        if id >= input.ring.size {
            continue;
        }
        if used.len as usize >= EVENT_BYTES {
            let raw = Raw::decode(input.ring.event(id));
            match input.report.feed(raw, &input.axes) {
                Step::Key(code, value) => kb.key(code, value),
                Step::Flush => post_pointer(&input.report.take()),
                Step::Gathered | Step::Ignored => {}
            }
        }
        input.ring.offer(id);
    }
    if seen > 0 {
        input.kick();
    }
    seen > 0
}
