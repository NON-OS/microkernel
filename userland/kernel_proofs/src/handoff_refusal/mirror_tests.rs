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

//! The loader's half and the kernel's agree: the flag, the codes, and every
//! word the loader can make decodes to what the loader meant.

use super::constants::flags as kernel;
use super::loader_constants::flags as loader;
use super::loader_word::{mask_bits, refusal_word};
use super::loader_word::{NO_FB_BAD_GEOMETRY, NO_FB_BITMASK, NO_FB_BLT_ONLY, NO_FB_NO_GOP};
use super::refusal::{self, refusal as decode, FbRefusal};

#[test]
fn the_flag_is_bit_16_on_both_sides_and_every_other_flag_still_agrees() {
    assert_eq!((kernel::FB_REFUSED, loader::FB_REFUSED), (1 << 16, 1 << 16));
    let pairs = [
        (kernel::FB_AVAILABLE, loader::FB_AVAILABLE),
        (kernel::PROFILE_RECOVERY, loader::PROFILE_RECOVERY),
    ];
    assert!(pairs.iter().all(|(k, l)| k == l));
    assert_eq!(kernel::flag_names(kernel::FB_REFUSED).len(), 17);
    assert_eq!(kernel::flag_names(u64::MAX).last(), Some(&"FB-REFUSED"));
}

#[test]
fn the_codes_are_the_same_numbers() {
    let k = [
        refusal::NO_FB_NO_GOP,
        refusal::NO_FB_BLT_ONLY,
        refusal::NO_FB_BITMASK,
        refusal::NO_FB_BAD_GEOMETRY,
    ];
    assert_eq!(k, [NO_FB_NO_GOP, NO_FB_BLT_ONLY, NO_FB_BITMASK, NO_FB_BAD_GEOMETRY]);
}

#[test]
fn every_word_the_loader_makes_decodes_to_what_it_meant() {
    for code in [NO_FB_NO_GOP, NO_FB_BLT_ONLY, NO_FB_BITMASK, NO_FB_BAD_GEOMETRY] {
        for bpp in 0..=255u8 {
            let got = decode(loader::FB_REFUSED, refusal_word(code, bpp)).unwrap().unwrap();
            let want = match code {
                NO_FB_NO_GOP => FbRefusal::NoGop,
                NO_FB_BLT_ONLY => FbRefusal::BltOnly,
                NO_FB_BITMASK => FbRefusal::Bitmask { bpp },
                _ => FbRefusal::BadGeometry,
            };
            assert_eq!(got, want, "code {code} bpp {bpp}");
        }
    }
}

#[test]
fn a_bitmask_modes_depth_is_every_bit_its_masks_name() {
    assert_eq!(mask_bits(0xF800, 0x07E0, 0x001F, 0), 16, "RGB565");
    assert_eq!(mask_bits(0x3FF0_0000, 0x000F_FC00, 0x0000_03FF, 0xC000_0000), 32, "10-bit");
    assert_eq!(mask_bits(0xFF_0000, 0x00_FF00, 0x00_00FF, 0), 24, "24-bit packed");
}
