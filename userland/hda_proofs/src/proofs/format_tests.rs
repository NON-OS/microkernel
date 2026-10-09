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
//! The stream format word both the controller and the converter decode.

use crate::controller::codec::format::{encode, PLAYBACK};

#[test]
fn the_playback_format_is_48k_16_bit_stereo() {
    assert_eq!(PLAYBACK, 0x0011);
}

#[test]
fn each_rate_uses_the_base_multiplier_and_divisor_the_specification_gives() {
    assert_eq!(encode(44_100, 16, 2), Some(0x4011));
    assert_eq!(encode(96_000, 24, 2), Some(0x0831));
    assert_eq!(encode(192_000, 32, 8), Some(0x1847));
    assert_eq!(encode(32_000, 16, 2), Some(0x0a11), "32 kHz is 48 x 2 / 3");
    assert_eq!(encode(8_000, 8, 1), Some(0x0500), "8 kHz is 48 / 6");
    assert_eq!(encode(22_050, 16, 2), Some(0x4111));
    assert_eq!(encode(176_400, 20, 2), Some(0x5821));
}

#[test]
fn what_cannot_be_encoded_is_refused() {
    assert_eq!(encode(12_345, 16, 2), None);
    assert_eq!(encode(48_000, 12, 2), None);
    assert_eq!(encode(48_000, 16, 0), None);
    assert_eq!(encode(48_000, 16, 17), None);
}
