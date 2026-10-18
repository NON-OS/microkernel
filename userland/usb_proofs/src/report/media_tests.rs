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

//! A keyboard's media keys, on an interface of their own: each key goes
//! down, repeats if it is a volume step, and comes up, through the same
//! keyboard state as any key; consumer and system keys keep apart.

use super::layout;
use super::vectors::MEDIA_KEYS;
use crate::endpoints::posted;
use crate::hid::ReportDevice;
use nonos_libc::{reset, INPUT_KIND_KEY_DOWN as DOWN, INPUT_KIND_KEY_UP as UP};

/// The PS/2 keycode table's mute, volume and power codes.
const MUTE: u32 = 0x1301;
const VOLUME_UP: u32 = 0x1303;
const POWER: u32 = 0x1304;

#[test]
fn volume_up_held_repeats_and_comes_up() {
    reset();
    let mut dev = ReportDevice::new(layout(MEDIA_KEYS));
    dev.feed(&[2, 0xe9, 0x00]);
    for t in 0..=520 {
        dev.tick(t);
    }
    dev.feed(&[2, 0, 0]);
    assert_eq!(posted(), vec![(DOWN, VOLUME_UP), (DOWN, VOLUME_UP), (UP, VOLUME_UP)]);
}

#[test]
fn mute_and_power_act_once_and_keep_apart() {
    reset();
    let mut dev = ReportDevice::new(layout(MEDIA_KEYS));
    dev.feed(&[3, 0x01]);
    dev.feed(&[2, 0xe2, 0x00]);
    for t in 0..=2000 {
        dev.tick(t);
    }
    // A consumer report releases no system key, nor one the other way.
    dev.feed(&[2, 0, 0]);
    dev.feed(&[3, 0x00]);
    assert_eq!(posted(), vec![(DOWN, POWER), (DOWN, MUTE), (UP, MUTE), (UP, POWER)]);
}

#[test]
fn reports_of_other_ids_or_cut_short_are_nothing() {
    reset();
    let mut dev = ReportDevice::new(layout(MEDIA_KEYS));
    for raw in [&[][..], &[2], &[2, 0xe9], &[1, 0xe9, 0], &[4, 0xff, 0xff], &[0xff, 0xe9, 0]] {
        dev.feed(raw);
    }
    assert_eq!(posted(), vec![]);
    dev.feed(&[2, 0xe9, 0]);
    dev.release_all();
    assert_eq!(posted(), vec![(DOWN, VOLUME_UP), (UP, VOLUME_UP)], "unplugged mid press");
}
