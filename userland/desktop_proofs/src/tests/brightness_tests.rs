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

//! The brightness keys in the shell: each is a system key, and what the
//! compositor answers becomes one notice that fits a toast whole and
//! replaces the last brightness notice.

use nonos_app_skeleton::{KEY_BRIGHTNESS_DOWN, KEY_BRIGHTNESS_UP};

use crate::shell_system_keys::brightness::{heard, is_brightness_notice, notice, Lit};
use crate::shell_system_keys::system_key::{system_key, SystemKey};
use crate::toast_state::toast::TOAST_TEXT_MAX;

fn said(lit: Lit) -> String {
    let mut out = [0u8; TOAST_TEXT_MAX];
    let n = notice(lit, &mut out);
    String::from_utf8(out[..n].to_vec()).unwrap()
}

#[test]
fn the_brightness_keys_are_system_keys() {
    assert_eq!(system_key(KEY_BRIGHTNESS_UP), Some(SystemKey::Brightness(true)));
    assert_eq!(system_key(KEY_BRIGHTNESS_DOWN), Some(SystemKey::Brightness(false)));
}

#[test]
fn the_answer_becomes_the_level_no_panel_or_a_failure() {
    assert_eq!(heard(Some((0, 60))), Lit::Level(60));
    assert_eq!(heard(Some((0, 400))), Lit::Level(100), "never past full");
    assert_eq!(heard(Some((-2, 0))), Lit::NoPanel);
    assert_eq!(heard(Some((-5, 0))), Lit::Failed);
    assert_eq!(heard(None), Lit::Failed, "no answer");
}

#[test]
fn each_notice_fits_and_is_known_for_a_brightness_notice() {
    assert_eq!(said(Lit::Level(5)), "Brightness 5%");
    assert_eq!(said(Lit::Level(60)), "Brightness 60%");
    assert_eq!(said(Lit::Level(100)), "Brightness 100%");
    for v in 0..=100 {
        assert!(is_brightness_notice(said(Lit::Level(v)).as_bytes()));
    }
    for lit in [Lit::NoPanel, Lit::Failed] {
        let s = said(lit);
        assert!(s.len() < TOAST_TEXT_MAX && is_brightness_notice(s.as_bytes()), "{s}");
    }
    assert!(!is_brightness_notice(b"Volume 40%") && !is_brightness_notice(b"Muted"));
}
