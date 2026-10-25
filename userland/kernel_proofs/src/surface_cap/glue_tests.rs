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

//! Where the cap is used: the register path takes it from the validated
//! boot framebuffer, validates the whole range in slices usercopy takes,
//! answers ENOMEM rather than panicking when the frame list will not fit,
//! and the compositor names a refusal with the same ceiling.

use super::cap::CEILING;

const FRAMES: &str =
    include_str!("../../../../src/syscall/dispatch/router/surface_handlers/frames.rs");
const STATE: &str = include_str!("../../../../src/kernel_core/init/framebuffer/state.rs");
const REJECTED: &str = include_str!("../../../compositor/src/setup/rejected.rs");
const BACKING: &str = include_str!("../../../compositor/src/setup/gop_backing.rs");

#[test]
fn the_cap_comes_from_the_validated_framebuffer() {
    // Through the framebuffer module's geometry accessor, never its state: the
    // console gate pins who reads that (console_gate::glue_writers_tests).
    assert!(FRAMES.contains("let fb = boot_frame_len().map(|n| n as u64);"));
    assert!(!FRAMES.contains("framebuffer_state"), "the register path reads the console's state");
    let accessor = STATE.find("pub(crate) fn boot_frame_len() -> Option<usize> {").expect("no accessor");
    assert!(STATE[accessor..].starts_with("pub(crate) fn boot_frame_len() -> Option<usize> {\n    framebuffer_state().and_then(|f| f.frame_len())"));
    assert!(FRAMES.contains("desc.byte_len > surface_cap(fb)"));
    assert!(!FRAMES.contains("MAX_SURFACE_BYTES"), "a fixed cap left beside the rule");
}

#[test]
fn the_whole_range_is_validated_in_slices_before_any_translation() {
    let end = FRAMES.find("if base.checked_add(desc.byte_len).is_none() {").expect("end unchecked");
    let slice = FRAMES.find("for (start, len) in slices(base, desc.byte_len, SLICE) {").expect("no slices");
    let walk = FRAMES.find("translate_address(va)").expect("no translation");
    assert!(end < slice && slice < walk, "{end} {slice} {walk}");
    assert!(FRAMES.find("desc.byte_len == 0 ||").expect("an empty surface admitted") < end);
    assert!(FRAMES.contains("let Some(pa) = translate_address(va) else {\n            return Err(errno(EFAULT));"));
}

#[test]
fn a_frame_list_that_will_not_fit_is_enomem() {
    assert!(FRAMES.contains("if frames.try_reserve_exact(pages).is_err() {"));
    assert!(FRAMES.contains("return Err(errno(ENOMEM));"));
    assert!(!FRAMES.contains("Vec::with_capacity"), "an allocation that panics on failure");
}

#[test]
fn the_compositor_names_the_kernels_ceiling_once() {
    let mirrored = "pub const SURFACE_CEILING: u64 = 128 * 1024 * 1024;";
    assert!(REJECTED.contains(mirrored));
    assert_eq!(CEILING, 128 * 1024 * 1024, "the compositor's mirror is stale");
    let said = BACKING.find("super::rejected::say_once(byte_len, sid, &super::rejected::SAID").expect("unsaid");
    assert!(said < BACKING.find("return Err(\"gop: surface register rejected\");").unwrap());
}
