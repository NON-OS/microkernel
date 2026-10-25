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

//! The decoder, over every word the handoff can carry.

use super::constants::flags::{FB_AVAILABLE, FB_REFUSED};
use super::refusal::{refusal, FbRefusal, Malformed, WHY_NONE_GIVEN, WHY_UNKNOWN};

fn refused(word: u32) -> FbRefusal {
    refusal(FB_REFUSED, word).expect("well formed").expect("no framebuffer")
}

#[test]
fn every_code_byte_with_every_depth_decodes_to_its_reason_or_unknown() {
    for word in 0u32..=0xFFFF {
        let (code, bpp) = (word & 0xFF, (word >> 8) as u8);
        let want = match (code, bpp) {
            (0, 0) => FbRefusal::NoGop,
            (1, 0) => FbRefusal::BltOnly,
            (2, bpp) => FbRefusal::Bitmask { bpp },
            (3, 0) => FbRefusal::BadGeometry,
            _ => FbRefusal::Unknown(word),
        };
        assert_eq!(refused(word), want, "{word:#x}");
    }
}

#[test]
fn a_stray_bit_above_the_depth_is_unknown_whatever_the_code() {
    for bit in 16..32 {
        for code in [0u32, 1, 2, 3, 0x0802] {
            let word = code | 1 << bit;
            assert_eq!(refused(word), FbRefusal::Unknown(word), "{word:#x}");
        }
    }
}

#[test]
fn both_flags_are_malformed_neither_is_no_reason_and_a_framebuffer_is_none() {
    for word in [0u32, 1, 0x1002, 0xFFFF_FFFF] {
        assert_eq!(refusal(FB_AVAILABLE | FB_REFUSED, word), Err(Malformed));
        assert_eq!(refusal(0, word), Ok(Some(FbRefusal::NoneGiven)), "an older loader");
        assert_eq!(refusal(FB_AVAILABLE, word), Ok(None));
    }
}

#[test]
fn userland_is_told_only_checked_words() {
    for word in (0u32..=0xFFFF).chain([1 << 16, 0xFFFF_FFFF]) {
        let told = refused(word).why_word();
        let (code, bpp) = (told & 0xFF, told >> 8);
        assert!(
            told >> 16 == 0 && matches!(code, 0..=3) || told == WHY_UNKNOWN as u32,
            "{told:#x}"
        );
        assert!(bpp == 0 || code == 2, "a depth with a reason that has none: {told:#x}");
    }
    assert_eq!(FbRefusal::NoneGiven.why_word(), WHY_NONE_GIVEN as u32);
    assert_eq!(FbRefusal::Bitmask { bpp: 16 }.why_word(), 0x1002);
}
