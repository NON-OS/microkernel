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

//! The five kernel draws each keep their guard for the whole draw: bound
//! once at the top of the function, so it lives to the function's end, never
//! dropped, forgotten or rebound, and every pixel written after it from the
//! framebuffer it gives.

use super::text::{code, depth};
use super::text_items::{at, body, flat};

/// File, function, guard, its binding, and what must come after it.
struct Site(&'static str, &'static str, &'static str, &'static str, &'static [&'static str]);

const HEART: &str = "let Some(fb) = crate::kernel_core::init::framebuffer::drawable() else {";
const PIXELS: &[&str] = &[
    "let base = (fb.base_va.as_u64() + fb.offset as u64) as *mut u32;",
    "core::ptr::write_volatile(base.add(off as usize), color)",
];
const LOG_BIND: &str = "let Some(draw) = drawable() else {";
const LOG: &[&str] = &["let fb: &KernelFramebuffer = &draw;", "fill_rect(fb,", "draw_char(fb,"];
pub const BLIT_RS: &str = "syscall/dispatch/router/graphics_present/blit.rs";
const BLIT: &[&str] = &[
    "let dst = (fb.base_va.as_u64() + fb.offset as u64) as *mut u8;",
    "super::store::store(dst.add(dst_off), &bounce.0[..chunk])",
];

const SITES: [Site; 5] = [
    Site("sys/boot_log/render.rs", "fn write_line(", "draw", LOG_BIND, LOG),
    Site(BLIT_RS, "fn blit(", "fb", "let Some(fb) = framebuffer::drawable() else {", BLIT),
    Site("interrupts/timer/heartbeat/on_tick.rs", "fn on_tick(", "fb", HEART, PIXELS),
    Site("interrupts/timer/heartbeat/paint_blip.rs", "fn paint_blip(", "fb", HEART, PIXELS),
    Site("interrupts/timer/heartbeat/draw_bar.rs", "fn draw_bar(", "fb", HEART, PIXELS),
];

#[test]
fn each_draw_keeps_its_guard_until_its_last_pixel() {
    for Site(rel, head, guard, bind, after) in &SITES {
        let file = code(rel);
        assert_eq!(file.matches("drawable(").count(), 1, "{rel}: the gate asked twice");
        let b = flat(body(&file, head));
        let bound = at(&b, &flat(bind));
        assert_eq!(depth(&b, bound), 0, "{rel}: the guard bound in an inner block");
        for later in *after {
            assert!(at(&b, &flat(later)) > bound, "{rel}: `{later}` before the guard");
        }
        for early in ["drop(", "forget(", "ManuallyDrop", "Box::leak"] {
            assert!(!b.contains(early), "{rel}: `{early}` lets the guard go early");
        }
        let binds = ["let{}=", "let{}:", "letmut{}", "({})"].map(|n| n.replace("{}", guard));
        let binds: usize = binds.iter().map(|n| b.matches(n.as_str()).count()).sum();
        assert_eq!(binds, 1, "{rel}: `{guard}` rebound or moved");
    }
}
