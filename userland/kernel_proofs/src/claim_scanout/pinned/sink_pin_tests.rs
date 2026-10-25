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

//! The sink and the geometry call read whole (L79). A taken-back holder's
//! loads read only what it stored in the sink since the takeover, which
//! needs the frame zeroed before any page can show it; and once the scanout
//! is taken nobody is told the framebuffer's geometry again (ENODEV).

use super::super::glue_text::{body, count, in_order, source};

const SINK: &str = "hardware/broker/bootfb/sink.rs";
const INFO: &str = "hardware/broker/bootfb/info.rs";

#[test]
fn the_sink_is_zeroed_before_it_is_published_and_never_freed() {
    let want = "{ let Some(frame) = frame_alloc::allocate_frame() else { return false; }; \
        frame_alloc::zero_frame(frame); SINK.call_once(|| frame); true }";
    assert_eq!(body(SINK, "make"), want);
    let text = source(SINK);
    assert!(!text.contains("free"), "the sink handed back");
    assert_eq!(count(&text, "SINK.call_once("), 1, "the sink made twice");
}

#[test]
fn the_geometry_is_refused_to_everyone_once_the_scanout_is_taken() {
    let f = body(INFO, "info_for");
    let first = "{ if !is_boot_framebuffer(device_id) || surrendered() \
        { return Err(BootFbInfoError::NotBootFramebuffer); }";
    assert!(f.starts_with(first), "the takeover's refusal moved: {f}");
    in_order(&f, &[first, "let held = claim::lookup(device_id)", "if !held {", "Ok(encode("]);
    let using = "use crate::kernel_core::init::framebuffer::{framebuffer_state, phys_window, \
        surrendered};";
    assert!(source(INFO).contains(using), "another `surrendered` asked");
}
