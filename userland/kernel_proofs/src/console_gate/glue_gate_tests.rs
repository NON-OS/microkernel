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

//! drawing.rs read: `drawable()`, the guard's drop and the wait are the
//! model's drawer steps and wait in the model's order, all SeqCst.

use super::text::code;
use super::text_items::{flat, flat_body};

const DRAWING: &str = "kernel_core/init/framebuffer/drawing.rs";

/// The file's one ordering is SeqCst, imported once, and no weaker one is
/// named; its statics are private and not mutable.
pub fn seqcst_only(rel: &str) -> String {
    let text = flat(&code(rel));
    for weak in ["Relaxed", "Acquire", "Release", "AcqRel", "staticmut", "pubstatic"] {
        assert!(!text.contains(weak), "{rel}: `{weak}`");
    }
    assert_eq!(text.matches("Ordering::").count(), 1, "{rel}: another ordering");
    assert!(text.contains("Ordering::SeqCst};"), "{rel}: the ordering is not SeqCst");
    text
}

#[test]
fn drawable_checks_counts_checks_again_and_backs_out() {
    let steps = [
        "ifstopped(){returnNone;}",
        "DRAWS.fetch_add(1,SeqCst);",
        "letfb=framebuffer_state().filter(|_|!stopped());",
        "iffb.is_none(){DRAWS.fetch_sub(1,SeqCst);}",
        "fb.map(Drawing)",
    ];
    assert_eq!(flat_body(DRAWING, "fn drawable()"), steps.concat());
}

#[test]
fn a_draw_ends_with_its_fence_then_its_decrement_and_a_wait_spins_on_draws() {
    let drop = "core::sync::atomic::fence(SeqCst);DRAWS.fetch_sub(1,SeqCst);";
    assert_eq!(flat_body(DRAWING, "fn drop(&mut self)"), drop);
    let wait = "whileDRAWS.load(SeqCst)!=0{crate::smp::serve_shootdowns();";
    let wait = format!("{wait}core::hint::spin_loop();}}");
    assert_eq!(flat_body(DRAWING, "fn wait_quiet()"), wait);
}

#[test]
fn draws_is_counted_only_by_those_steps_and_only_with_seqcst() {
    let text = seqcst_only(DRAWING);
    assert!(text.contains("staticDRAWS:AtomicU32=AtomicU32::new(0);"));
    assert_eq!(text.matches("DRAWS.").count(), 4, "DRAWS touched elsewhere");
    assert_eq!(text.matches("SeqCst)").count(), 5, "an atomic step not in the model");
}
