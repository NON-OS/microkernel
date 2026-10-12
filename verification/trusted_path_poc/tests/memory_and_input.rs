// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #400 the fixed allocator hands out no aliased frame and has no fallback
//! source; #401 draining is strictly narrower than posting, so an interrupt-only
//! driver can post input but never drain it.

use nonos_trusted_path_poc::{frame_alloc, input_authority};

#[test]
fn fixed_allocator_returns_none_on_exhaustion() {
    assert_eq!(frame_alloc::fixed_alloc(None), None);
}

#[test]
fn old_allocator_handed_out_an_aliasing_bump_frame() {
    assert_eq!(frame_alloc::old_alloc(None, 16), Some(16));
    assert!(frame_alloc::bump_aliases(&[16, 32, 48], 16, 0));
    assert_eq!(frame_alloc::fixed_alloc(None), None);
}

#[test]
fn interrupt_or_admin_alone_neither_posts_nor_drains() {
    for (irq, admin) in [(true, false), (false, true), (true, true)] {
        let r = input_authority::Rights { input_source: false, input_consumer: false, irq, admin };
        assert!(!input_authority::can_post(r), "Irq or Admin must not post input");
        assert!(!input_authority::can_drain(r), "Irq or Admin must not drain the input ring");
    }
}

#[test]
fn each_gate_is_its_own_right_for_all_rights() {
    for bits in 0u8..16 {
        let r = input_authority::Rights {
            input_source: bits & 1 != 0,
            input_consumer: bits & 2 != 0,
            irq: bits & 4 != 0,
            admin: bits & 8 != 0,
        };
        assert_eq!(input_authority::can_post(r), r.input_source);
        assert_eq!(input_authority::can_drain(r), r.input_consumer);
    }
}
