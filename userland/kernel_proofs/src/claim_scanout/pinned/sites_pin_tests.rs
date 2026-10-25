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

//! The five kernel draw sites read (L79): each takes the framebuffer only
//! from `drawable`, bound to a name that lives to the end of the function,
//! so the count the takeover waits on covers every store; none hands the
//! guard on, drops it, or reads the framebuffer past the gate. The model's
//! drawer is one such site; a site that let its guard go before its stores
//! would draw under a GPU made ready with every other check passing.

use super::super::glue_text::{at, count, source};

const GATE: &str = "crate::kernel_core::init::framebuffer::drawable() else { return; };";

/// Each site, its binding of the gate, and the first store after it.
const SITES: [(&str, &str, &str); 5] = [
    (
        "sys/boot_log/render.rs",
        "let Some(draw) = drawable() else { return; }; let fb: &KernelFramebuffer = &draw;",
        "draw_char(fb,",
    ),
    (
        "syscall/dispatch/router/graphics_present/blit.rs",
        "let Some(fb) = framebuffer::drawable() else { let held = \
      framebuffer::framebuffer_state().is_some(); \
      return util::errno(if held { EBUSY } else { ENOTSUP }); };",
        "super::store::store(",
    ),
    ("interrupts/timer/heartbeat/on_tick.rs", GATE, "write_volatile("),
    ("interrupts/timer/heartbeat/paint_blip.rs", GATE, "write_volatile("),
    ("interrupts/timer/heartbeat/draw_bar.rs", GATE, "write_volatile("),
];

#[test]
fn every_kernel_draw_holds_its_count_across_its_stores() {
    for (rel, gate, store) in SITES {
        let text = source(rel);
        assert_eq!(count(&text, "drawable()"), 1, "{rel}: another way in");
        let bound = if gate == GATE { format!("let Some(fb) = {GATE}") } else { gate.into() };
        let from = at(&text, &bound);
        assert!(at(&text, store) > from, "{rel}: a store before the gate");
        // Only the present's EBUSY answer looks at the state, and draws nothing.
        let states = usize::from(rel.ends_with("blit.rs"));
        assert_eq!(count(&text, "framebuffer_state()"), states, "{rel}: past the gate");
        for gone in ["drop(fb)", "drop(draw)", "mem::forget", "fb.0", "draw.0"] {
            assert!(!text.contains(gone), "{rel}: the guard let go by `{gone}`");
        }
    }
}
