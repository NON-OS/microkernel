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

//! The takeover and its take-back read whole (L79). The first checks found
//! each step after the one before, which let a line slip in between: the
//! stop undone as soon as it was made passed them, and the kernel drew on
//! through the whole takeover. Here each body is the kernel's to the end.

use super::super::glue_text::body;

const SCANOUT: &str = "hardware/broker/claim/scanout.rs";
const TAKE_BACK: &str = "hardware/broker/mmio/take_back.rs";

#[test]
fn the_takeover_and_its_take_back_are_the_kernels_to_the_last_token() {
    let take = [
        "{ console::stop_drawing();",
        "let (Some(fb), Some(sink)) = (bootfb::device(), bootfb::sink()) else { return; };",
        "loop { let seen = peek(fb); match (next(seen.map(|s| s.1)), seen) {",
        "(Next::TakeBack, Some((pid, _))) => take_back(by, pid, fb, sink),",
        "(Next::Done, _) | (Next::TakeBack, None) => return,",
        "(Next::Wait, _) => { crate::smp::serve_shootdowns(); core::hint::spin_loop(); } } } }",
    ];
    assert_eq!(body(SCANOUT, "take"), take.join(" "));
    let back = [
        "{ let Ok(epoch) = super::leave::retire(pid, fb) else { return; };",
        "let pages = mmio::take_back(pid, fb, epoch, sink);",
        "let _ = super::release::release(pid, fb, epoch);",
        "let why = \"takes the boot framebuffer's scanout: taken from pid\";",
    ];
    assert!(body(SCANOUT, "take_back").starts_with(&back.join(" ")), "the take-back changed");
    let pages = [
        "{ let drained = grant::drain_for_device(holder, device_id, epoch);",
        "let Some(asid) = crate::kernel_core::surface_registry::space_of(holder) else",
        "{ return 0; }; let moved = |g: &grant::MmioGrant| {",
        "let (va, from) = (VirtAddr::new(g.user_va), PhysAddr::new(g.physical_start));",
        "repoint_user_mmio(asid, va, pages_needed(g.length as usize), from, sink) };",
        "drained.iter().map(moved).sum() }",
    ];
    assert_eq!(body(TAKE_BACK, "take_back"), pages.join(" "));
}
