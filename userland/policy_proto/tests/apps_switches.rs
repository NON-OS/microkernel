/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The app switches: one bit each, the dock services each covers, and the
 * exit status that carries them from setup to the kernel.
 */

use nonos_policy_proto::apps::{bit_for_service, exit_parts, exit_status, is_off, OPTIONAL};
use nonos_policy_proto::apps::{BROWSER, LINUX, MEDIA, NONE_OFF, STORE};

#[test]
fn each_switch_has_its_own_bit_and_all_eight_are_used() {
    let all = OPTIONAL.iter().fold(0u8, |seen, a| {
        assert_eq!(a.bit.count_ones(), 1, "{:?}", a.name);
        assert_eq!(seen & a.bit, 0, "{:?} shares a bit", a.name);
        seen | a.bit
    });
    assert_eq!(all, 0xFF);
}

#[test]
fn dock_services_map_to_their_switch() {
    assert_eq!(bit_for_service(b"app.browser"), BROWSER);
    assert_eq!(bit_for_service(b"app.store"), STORE);
    assert_eq!(bit_for_service(b"app.video_player"), MEDIA);
    assert_eq!(bit_for_service(b"app.image_viewer"), MEDIA);
    assert_eq!(bit_for_service(b"app.terminal"), 0, "required apps have no switch");
    assert_eq!(bit_for_service(b"app.settings"), 0);
    assert!(is_off(MEDIA, b"app.audio_player"));
    assert!(!is_off(BROWSER, b"app.audio_player"));
    assert!(!is_off(0xFF, b"app.terminal"), "nothing turns a required app off");
    assert_eq!(bit_for_service(b"tool.qwen"), LINUX, "the dock's Qwen goes with Linux");
    assert!(OPTIONAL.iter().any(|a| a.bit == LINUX && a.services == [&b"tool.qwen"[..]]));
}

#[test]
fn the_exit_status_carries_what_to_start_and_the_apps_off() {
    assert_eq!(exit_status(0, NONE_OFF), 0, "every app on is the old status");
    assert_eq!(exit_status(3, NONE_OFF), 3);
    for (what, off) in [(0, BROWSER | LINUX), (3, 0xFF), (2, 0)] {
        assert_eq!(exit_parts(exit_status(what, off)), Some((what, off)));
    }
}

#[test]
fn a_status_setup_does_not_write_carries_nothing() {
    assert_eq!(exit_parts(-1), None);
    assert_eq!(exit_parts(1 << 16), None);
    assert_eq!(exit_parts(i32::MIN), None);
}
