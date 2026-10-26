// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Recency is the shell's alone, and the shell is who the registry names,
//! a name no capsule can take: the kernel's own tables say every grabber's
//! name is the image's, and none may be registered at run time. A capsule
//! the user pressed last gains nothing, and neither does any name's claim.

use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_KEY_DOWN, INPUT_KIND_POINTER_REL};
use nonos_libc::{INPUT_KIND_KEY_UP, INPUT_KIND_TOUCH};

use crate::tests::kernel_reserved::{is_reserved_service, is_runtime_registrable};
use crate::tests::kernel_system_names::{is_reserved_form, is_system_name};
use crate::route::grab_rule::{may_grab, GRABBERS};
use crate::route::pad::PAD_BUTTON_DOWN;
use crate::route::recent::{after, is_press};
use crate::router_desk::SHELL;

const A: u32 = 20;
const B: u32 = 21;

#[test]
fn recency_counts_for_the_shell_only() {
    assert!(may_grab(SHELL, true, Some(A), SHELL, SHELL));
    assert!(!may_grab(B, true, Some(A), SHELL, B), "B was pressed last, but is not the shell");
    assert!(!may_grab(B, true, Some(A), SHELL, SHELL), "the shell's press is not B's");
    assert!(!may_grab(SHELL, false, Some(A), SHELL, SHELL), "unnamed");
    assert!(!may_grab(SHELL, true, Some(A), SHELL, A), "the window had the last input");
    assert!(!may_grab(0, true, Some(A), 0, 0), "no shell");
}

#[test]
fn the_record_is_what_the_router_delivered() {
    for kind in [INPUT_KIND_KEY_DOWN, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_TOUCH, PAD_BUTTON_DOWN] {
        assert!(is_press(kind));
        assert_eq!(after(SHELL, A, kind, 1), A, "delivered to A");
        assert_eq!(after(SHELL, A, kind, 0), 0, "lost on the way: nobody's");
    }
    for kind in [INPUT_KIND_POINTER_REL, INPUT_KIND_KEY_UP] {
        assert_eq!(after(SHELL, A, kind, 1), SHELL, "kind {kind} aims at nothing");
    }
}

#[test]
fn every_grabber_name_is_the_images_and_none_is_taken_at_run_time() {
    assert!(GRABBERS.contains(&&b"desktop_shell"[..]));
    for name in GRABBERS {
        let name = core::str::from_utf8(name).expect("ascii");
        assert!(is_system_name(name), "{name}: a publisher's capsule may not take it");
        assert!(!is_runtime_registrable(name), "{name}: nor any capsule at run time");
        assert!(!is_reserved_service(name, 1) && !is_reserved_form(name), "{name}: by its own list");
    }
}
