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

//! The volume and power keys in the shell: the codes it answers are the ones
//! the router sends it, a press steps and clamps the level or flips mute and
//! asks the audio service for exactly that, and the notice says what plays
//! in one toast that the next press replaces.

use nonos_app_skeleton::{KEY_ESC, KEY_F1, KEY_MUTE, KEY_POWER, KEY_VOLUME_DOWN, KEY_VOLUME_UP};
use nonos_audio_proto::{
    read_volume_request, MasterVolume, HDR_LEN, OP_SET_VOLUME, OUTPUT_NEEDS_SOF, OUTPUT_NO_DEVICE,
    VOLUME_MAX, VOLUME_MSG_LEN,
};

use crate::router_shell_keys::{
    is_shell_key, KEY_MUTE as ROUTER_MUTE, KEY_POWER as ROUTER_POWER,
    KEY_VOLUME_DOWN as ROUTER_DOWN, KEY_VOLUME_UP as ROUTER_UP,
};
use crate::shell_system_keys::system_key::{system_key, SystemKey, POWER_OFF_UNAVAILABLE};
use crate::shell_system_keys::volume::{
    after_key, is_volume_notice, notice, press, volume_key, Heard, VolumeKey, VOLUME_STEP,
};
use crate::toast_state::toast::{UptimeMs, TOAST_TEXT_MAX};
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

fn v(level: u8, muted: bool) -> MasterVolume {
    MasterVolume { level, muted }
}

fn said(heard: Heard) -> String {
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = notice(heard, &mut line);
    String::from_utf8(line[..n].to_vec()).unwrap()
}

#[test]
fn the_shell_answers_the_codes_the_router_hands_it() {
    assert_eq!(
        [KEY_MUTE, KEY_VOLUME_DOWN, KEY_VOLUME_UP, KEY_POWER],
        [ROUTER_MUTE, ROUTER_DOWN, ROUTER_UP, ROUTER_POWER]
    );
    // Every key the router sends here past focus is one the shell takes, so
    // none reaches a rename or the Launchpad search as a typed character.
    for code in 0..0x4000u32 {
        assert_eq!(is_shell_key(code), system_key(code).is_some(), "{code:#x}");
    }
    assert_eq!(system_key(KEY_MUTE), Some(SystemKey::Volume(VolumeKey::Mute)));
    assert_eq!(system_key(KEY_VOLUME_DOWN), Some(SystemKey::Volume(VolumeKey::Down)));
    assert_eq!(system_key(KEY_VOLUME_UP), Some(SystemKey::Volume(VolumeKey::Up)));
    assert_eq!(system_key(KEY_POWER), Some(SystemKey::Power));
    assert_eq!(volume_key(KEY_POWER), None);
    for code in [KEY_ESC, KEY_F1, u32::from(b'm'), u32::from(b'+')] {
        assert_eq!(system_key(code), None, "{code:#x}");
    }
}

#[test]
fn up_and_down_step_by_five_and_clamp() {
    assert_eq!(VOLUME_STEP, 5);
    assert_eq!(after_key(v(40, false), VolumeKey::Up), v(45, false));
    assert_eq!(after_key(v(40, false), VolumeKey::Down), v(35, false));
    assert_eq!(after_key(v(100, false), VolumeKey::Up), v(100, false), "clamped at full");
    assert_eq!(after_key(v(98, false), VolumeKey::Up), v(100, false));
    assert_eq!(after_key(v(0, false), VolumeKey::Down), v(0, false), "clamped at silence");
    assert_eq!(after_key(v(3, false), VolumeKey::Down), v(0, false));
}

#[test]
fn a_held_key_walks_the_whole_range_and_stops_at_each_end() {
    // A held key repeats as more presses, a step each.
    let mut now = v(0, false);
    for _ in 0..30 {
        now = after_key(now, VolumeKey::Up);
        assert!(now.level <= VOLUME_MAX);
    }
    assert_eq!(now, v(100, false));
    for _ in 0..30 {
        now = after_key(now, VolumeKey::Down);
    }
    assert_eq!(now, v(0, false));
    let presses = (0..).scan(v(0, false), |s, _| {
        *s = after_key(*s, VolumeKey::Up);
        Some(s.level)
    });
    assert_eq!(presses.take_while(|&l| l < 100).count() + 1, 20, "twenty steps to full");
}

#[test]
fn mute_flips_and_keeps_the_level_and_a_step_unmutes() {
    let muted = after_key(v(60, false), VolumeKey::Mute);
    assert_eq!(muted, v(60, true));
    assert_eq!(after_key(muted, VolumeKey::Mute), v(60, false));
    assert_eq!(after_key(muted, VolumeKey::Up), v(65, false));
    assert_eq!(after_key(muted, VolumeKey::Down), v(55, false));
}

#[test]
fn a_press_sends_the_volume_it_asks_for() {
    for (now, key) in [
        (v(40, false), VolumeKey::Up),
        (v(40, false), VolumeKey::Down),
        (v(40, false), VolumeKey::Mute),
        (v(100, false), VolumeKey::Up),
        (v(0, true), VolumeKey::Down),
    ] {
        let mut out = [0u8; VOLUME_MSG_LEN];
        let (want, n) = press(now, key, 2, &mut out);
        assert_eq!(want, after_key(now, key));
        assert_eq!(n, VOLUME_MSG_LEN);
        assert_eq!(u16::from_le_bytes([out[6], out[7]]), OP_SET_VOLUME);
        assert_eq!(u32::from_le_bytes([out[12], out[13], out[14], out[15]]), 2);
        assert_eq!(read_volume_request(&out[HDR_LEN..n]), Some(want), "{now:?} {key:?}");
    }
    let (_, n) = press(v(40, false), VolumeKey::Up, 2, &mut [0u8; VOLUME_MSG_LEN - 1]);
    assert_eq!(n, 0, "a buffer too short sends nothing");
}

#[test]
fn the_notice_says_the_level_or_muted_or_that_nothing_plays() {
    assert_eq!(said(Heard::Playing(v(45, false))), "Volume 45%");
    assert_eq!(said(Heard::Playing(v(100, false))), "Volume 100%");
    assert_eq!(said(Heard::Playing(v(5, false))), "Volume 5%");
    assert_eq!(said(Heard::Playing(v(0, false))), "Volume 0%");
    assert_eq!(said(Heard::Playing(v(45, true))), "Muted");
    assert_eq!(said(Heard::NoOutput(None)), "No sound output");
    assert_eq!(
        said(Heard::NoOutput(Some(OUTPUT_NEEDS_SOF))),
        "No sound output: Needs Intel SOF firmware"
    );
    assert_eq!(said(Heard::NoOutput(Some(OUTPUT_NO_DEVICE))), "No sound output: No sound hardware");
}

#[test]
fn every_notice_fits_a_toast_whole_and_is_known_for_a_volume_notice() {
    let mut all: Vec<Heard> = (0..=VOLUME_MAX)
        .flat_map(|l| [Heard::Playing(v(l, false)), Heard::Playing(v(l, true))])
        .collect();
    all.push(Heard::NoOutput(None));
    all.extend((0..=8).map(|code| Heard::NoOutput(Some(code))));
    all.push(Heard::NoOutput(Some(u32::MAX)));
    for heard in all {
        let mut wide = [0u8; 4 * TOAST_TEXT_MAX];
        let whole = notice(heard, &mut wide);
        assert!(whole <= TOAST_TEXT_MAX, "{heard:?} is cut in a toast");
        let line = said(heard);
        assert!(is_volume_notice(line.as_bytes()), "{line:?}");
        assert!(!line.contains('\u{2014}'), "{line:?}");
    }
    for other in [&b"network connected"[..], b"Terminal opened", POWER_OFF_UNAVAILABLE, b"Mute"] {
        assert!(!is_volume_notice(other), "{:?}", String::from_utf8_lossy(other));
    }
}

#[test]
fn the_power_key_says_the_desktop_cannot_power_off() {
    assert_eq!(POWER_OFF_UNAVAILABLE, b"Power off is not available from the desktop");
    assert!(POWER_OFF_UNAVAILABLE.len() <= TOAST_TEXT_MAX, "cut in a toast");
}

#[test]
fn a_volume_notice_replaces_the_last_one_and_leaves_the_rest_in_order() {
    let mut q = ToastQueue::new();
    let at = UptimeMs(1_000);
    q.push(b"network connected", NotifyLevel::Info, at);
    for level in [50, 55, 60] {
        let line = said(Heard::Playing(v(level, false)));
        q.replace(line.as_bytes(), NotifyLevel::Info, at, is_volume_notice);
    }
    q.push(b"Terminal opened", NotifyLevel::Info, at);
    let before = q.generation();
    q.replace(b"Muted", NotifyLevel::Info, at, is_volume_notice);
    assert_ne!(q.generation(), before, "the panel is repainted for the change");
    let live: Vec<String> =
        q.iter_live().map(|t| String::from_utf8(t.text[..t.len].to_vec()).unwrap()).collect();
    assert_eq!(live, ["network connected", "Terminal opened", "Muted"]);
}

#[test]
fn a_held_key_keeps_its_notice_up_from_the_last_press() {
    let mut q = ToastQueue::new();
    q.replace(b"Volume 50%", NotifyLevel::Info, UptimeMs(0), is_volume_notice);
    q.replace(b"Volume 55%", NotifyLevel::Info, UptimeMs(2_000), is_volume_notice);
    q.expire(UptimeMs(3_000));
    let live: Vec<_> = q.iter_live().map(|t| t.text[..t.len].to_vec()).collect();
    assert_eq!(live, [b"Volume 55%".to_vec()], "up 2.5 s from the last press, not the first");
    // The same level again (Volume Up held at full) puts it up again too.
    q.replace(b"Volume 55%", NotifyLevel::Info, UptimeMs(4_000), is_volume_notice);
    q.expire(UptimeMs(5_000));
    assert_eq!(q.iter_live().count(), 1);
}
