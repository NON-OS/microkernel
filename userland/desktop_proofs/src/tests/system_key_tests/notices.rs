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

//! Volume notices fit a toast, replace each other and stay up while held.

use super::rig::{said, v};
use crate::shell_system_keys::system_key::POWER_OFF_UNAVAILABLE;
use crate::shell_system_keys::volume::{is_volume_notice, notice, Heard};
use crate::toast_state::toast::{UptimeMs, TOAST_TEXT_MAX};
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;
use nonos_audio_proto::VOLUME_MAX;

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
