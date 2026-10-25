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

//! Where the refusal is read and handed on, in the files the host cannot run.

const SECURITY: &str = include_str!("../../../../src/boot/handoff/api/security/framebuffer.rs");
const NO_FB: &str = include_str!("../../../../src/kernel_core/init/framebuffer/no_fb.rs");
const GDIM: &str = include_str!("../../../../src/syscall/dispatch/router/graphics_backend.rs");
const FLAGS: &str = include_str!("../../../../nonos-bootloader/src/handoff/prepare/flags.rs");
const GET_INFO: &str =
    include_str!("../../../../nonos-bootloader/src/handoff/config/framebuffer/get_info.rs");
const TRY_INIT: &str = include_str!("../../../../nonos-bootloader/src/display/gop/try_init.rs");

#[test]
fn a_handoff_claiming_both_is_refused_before_anything_reads_the_framebuffer() {
    let both = SECURITY
        .find("refusal(handoff.flags, handoff.fb.pixel_format).is_err()")
        .expect("unchecked");
    assert!(both < SECURITY.find("if !handoff.has_flag(flags::FB_AVAILABLE) {").unwrap());
    assert!(SECURITY.contains("return Err(reject(FbGeometryReason::RefusedAndAvailable));"));
}

#[test]
fn the_kernel_keeps_the_reason_and_says_it_once() {
    assert!(NO_FB.contains("REFUSAL.call_once(|| why.why_word());"));
    assert_eq!(NO_FB.matches("log_no_framebuffer(why);").count(), 1);
    assert!(NO_FB.contains("print(b\"[GOP] no framebuffer handed over\");"));
}

#[test]
fn gdim_tells_why_with_enotsup_and_zero_with_a_framebuffer() {
    let none = GDIM
        .find("let why = crate::kernel_core::init::framebuffer::boot_refusal();")
        .expect("unread");
    let enotsup =
        GDIM.find("return errno(if told { ENOTSUP } else { EFAULT });").expect("unanswered");
    assert!(none < enotsup);
    let told = "let told = out_why == 0 || write_user_value(out_why, &why).is_ok();";
    assert!(GDIM[none..enotsup].contains(told), "ENOTSUP with no reason written");
    assert!(GDIM.contains("(out_mm, mm), (out_why, 0)];"), "0 with a framebuffer");
    assert!(GDIM.contains("at != 0 && write_user_value(at, &v)"), "a pointer left 0 written");
}

#[test]
fn the_loader_sets_the_flag_with_no_framebuffer_and_hands_over_its_word() {
    assert!(FLAGS.contains(
        "        f |= flags::FB_AVAILABLE;\n    } else {\n        f |= flags::FB_REFUSED;\n    }"
    ));
    assert!(GET_INFO.contains("pixel_format: crate::display::gop::refusal(),"));
    let failure = TRY_INIT.find("FB_FAILURE.store(modes.failure(), Ordering::SeqCst);").unwrap();
    assert!(
        TRY_INIT[failure..].contains("FB_BITMASK_BPP.store(modes.bitmask_bpp, Ordering::SeqCst);")
    );
}
