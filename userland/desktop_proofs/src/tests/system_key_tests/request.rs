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

//! The request a press sends, and the notice it puts up.

use super::rig::{said, v};
use crate::shell_system_keys::volume::keys::after_key;
use crate::shell_system_keys::volume::{press, Heard, VolumeKey};
use nonos_audio_proto::{
    read_volume_request, HDR_LEN, OP_SET_VOLUME, OUTPUT_NEEDS_SOF, OUTPUT_NO_DEVICE, VOLUME_MSG_LEN,
};

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
