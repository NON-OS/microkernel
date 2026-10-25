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

//! The codes the router hands the shell, and the power key's word.

use crate::router_shell_keys::{
    is_shell_key, KEY_MUTE as ROUTER_MUTE, KEY_POWER as ROUTER_POWER,
    KEY_VOLUME_DOWN as ROUTER_DOWN, KEY_VOLUME_UP as ROUTER_UP,
};
use crate::shell_system_keys::system_key::{system_key, SystemKey, POWER_OFF_UNAVAILABLE};
use crate::shell_system_keys::volume::{volume_key, VolumeKey};
use crate::toast_state::toast::TOAST_TEXT_MAX;
use nonos_app_skeleton::{KEY_ESC, KEY_F1, KEY_MUTE, KEY_POWER, KEY_VOLUME_DOWN, KEY_VOLUME_UP};

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
fn the_power_key_says_the_desktop_cannot_power_off() {
    assert_eq!(POWER_OFF_UNAVAILABLE, b"Power off is not available from the desktop");
    assert!(POWER_OFF_UNAVAILABLE.len() <= TOAST_TEXT_MAX, "cut in a toast");
}
