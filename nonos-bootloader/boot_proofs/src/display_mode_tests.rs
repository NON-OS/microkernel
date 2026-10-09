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

//! The GOP mode a real laptop panel gets: the firmware's current mode or the
//! EDID native one, never a "largest under 1080p" guess that stretches a
//! 2560x1600 panel or the first mode, 640x480.

use crate::display::pick::{
    bitmask_bgr, choose, pack_mm, parse_edid, Choice, Offered, Source, FALLBACK_MAX_PIXELS,
};

fn offered(list: &[(u32, u32)]) -> Vec<Offered> {
    list.iter()
        .enumerate()
        .map(|(i, &(width, height))| Offered { index: i as u32, width, height })
        .collect()
}

/// What Intel GOP offers on a 2560x1600 eDP panel: the native mode plus the
/// usual scaled list, in no particular order.
const INTEL_2560: [(u32, u32); 6] =
    [(640, 480), (800, 600), (1024, 768), (1280, 1024), (1920, 1080), (2560, 1600)];

#[test]
fn the_firmwares_current_native_mode_is_kept() {
    let m = offered(&INTEL_2560);
    assert_eq!(choose(&m, Some((2560, 1600)), None, None), Some((Choice::Keep, Source::Current)));
    let hp = offered(&[(640, 480), (800, 600), (1024, 768), (1366, 768)]);
    assert_eq!(choose(&hp, Some((1366, 768)), None, None), Some((Choice::Keep, Source::Current)));
}

#[test]
fn a_panel_above_1080p_is_never_dropped_to_a_scaled_mode() {
    // The old rule skipped every mode past 1920x1080 and set the largest
    // left, so a 2560x1600 panel scanned out 1920x1080 through its scaler.
    let m = offered(&INTEL_2560);
    let edid = Some((2560, 1600));
    assert_eq!(choose(&m, Some((2560, 1600)), edid, None), Some((Choice::Keep, Source::Native)));
    assert_eq!(choose(&m, Some((1024, 768)), edid, None), Some((Choice::Set(5), Source::Native)));
}

#[test]
fn the_edid_native_mode_replaces_a_logo_mode() {
    let m = offered(&[(800, 600), (1024, 768), (1920, 1080)]);
    let got = choose(&m, Some((1024, 768)), Some((1920, 1080)), None);
    assert_eq!(got, Some((Choice::Set(2), Source::Native)));
}

#[test]
fn a_native_mode_the_gop_does_not_offer_keeps_the_current_one() {
    let m = offered(&[(1024, 768), (1280, 800)]);
    let got = choose(&m, Some((1280, 800)), Some((1920, 1200)), None);
    assert_eq!(got, Some((Choice::Keep, Source::Current)));
}

#[test]
fn a_text_console_default_gives_way_to_the_largest_mode() {
    let m = offered(&[(640, 480), (800, 600), (1024, 768), (1600, 900)]);
    assert_eq!(choose(&m, Some((800, 600)), None, None), Some((Choice::Set(3), Source::Largest)));
    // With an EDID the fallback never goes past the panel.
    let m = offered(&[(640, 480), (1366, 768), (1920, 1080)]);
    let got = choose(&m, Some((640, 480)), Some((1400, 900)), None);
    assert_eq!(got, Some((Choice::Set(1), Source::Largest)));
}

#[test]
fn the_fallback_has_a_memory_cap_but_the_native_mode_does_not() {
    let m = offered(&[(1920, 1080), (7680, 4320)]);
    const { assert!(7680u64 * 4320 > FALLBACK_MAX_PIXELS) };
    assert_eq!(choose(&m, None, None, None), Some((Choice::Set(0), Source::Largest)));
    assert_eq!(choose(&m, None, Some((7680, 4320)), None), Some((Choice::Set(1), Source::Native)));
}

#[test]
fn no_linear_mode_and_no_current_one_is_no_choice() {
    assert_eq!(choose(&[], None, Some((1920, 1080)), None), None);
    let m = offered(&[(1024, 768)]);
    assert_eq!(choose(&m, None, None, None), Some((Choice::Set(0), Source::Largest)));
}

#[test]
fn a_development_pin_wins_when_offered() {
    let m = offered(&[(1280, 800), (1920, 1080)]);
    let got = choose(&m, Some((1920, 1080)), Some((1920, 1080)), Some((1280, 800)));
    assert_eq!(got, Some((Choice::Set(0), Source::Pinned)));
    let got = choose(&m, Some((1920, 1080)), None, Some((3000, 2000)));
    assert_eq!(got, Some((Choice::Keep, Source::Current)));
}

#[test]
fn bitmask_modes_are_honoured_only_for_32_bit_rgb_and_bgr() {
    assert_eq!(bitmask_bgr(0x00FF_0000, 0x0000_FF00, 0x0000_00FF, 0xFF00_0000), Some(true));
    assert_eq!(bitmask_bgr(0x0000_00FF, 0x0000_FF00, 0x00FF_0000, 0xFF00_0000), Some(false));
    // 24 bpp packed: no reserved byte, so a 32 bit store would smear pixels.
    assert_eq!(bitmask_bgr(0x00FF_0000, 0x0000_FF00, 0x0000_00FF, 0), None);
    // RGB565 and 10 bit channels are other layouts entirely.
    assert_eq!(bitmask_bgr(0xF800, 0x07E0, 0x001F, 0), None);
    assert_eq!(bitmask_bgr(0x3FF0_0000, 0x000F_FC00, 0x0000_03FF, 0xC000_0000), None);
}

/// An EDID 1.4 base block for a panel, with a valid checksum.
fn edid(w: u32, h: u32, w_mm: u32, h_mm: u32, cm: (u8, u8), first_display: bool) -> [u8; 128] {
    let mut b = [0u8; 128];
    b[..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);
    b[18] = 1;
    b[19] = 4;
    b[21] = cm.0;
    b[22] = cm.1;
    let at = if first_display {
        // A display range descriptor first: pixel clock zero, tag 0xFD.
        b[54 + 3] = 0xFD;
        72
    } else {
        54
    };
    let d = &mut b[at..at + 18];
    d[0] = 0x1A; // 138.5 MHz, any non-zero clock marks a timing
    d[1] = 0x36;
    d[2] = (w & 0xFF) as u8;
    d[4] = (((w >> 8) & 0xF) << 4) as u8;
    d[5] = (h & 0xFF) as u8;
    d[7] = (((h >> 8) & 0xF) << 4) as u8;
    d[12] = (w_mm & 0xFF) as u8;
    d[13] = (h_mm & 0xFF) as u8;
    d[14] = ((((w_mm >> 8) & 0xF) << 4) | ((h_mm >> 8) & 0xF)) as u8;
    d[17] = 0x18;
    let sum = b[..127].iter().fold(0u8, |s, &v| s.wrapping_add(v));
    b[127] = 0u8.wrapping_sub(sum);
    b
}

#[test]
fn the_preferred_timing_is_the_native_mode_and_size() {
    let e = parse_edid(&edid(1920, 1080, 294, 165, (29, 17), false)).unwrap();
    assert_eq!((e.width, e.height, e.width_mm, e.height_mm), (1920, 1080, 294, 165));
    let e = parse_edid(&edid(2560, 1600, 286, 179, (29, 18), false)).unwrap();
    assert_eq!((e.width, e.height), (2560, 1600));
    let e = parse_edid(&edid(1366, 768, 344, 194, (34, 19), true)).unwrap();
    assert_eq!((e.width, e.height, e.width_mm), (1366, 768, 344));
}

#[test]
fn a_missing_millimetre_size_falls_back_to_centimetres_or_nothing() {
    let e = parse_edid(&edid(1920, 1080, 0, 0, (34, 19), false)).unwrap();
    assert_eq!((e.width_mm, e.height_mm), (340, 190));
    // Byte 22 zero: 21 is an aspect ratio, not a size.
    let e = parse_edid(&edid(1920, 1080, 0, 0, (79, 0), false)).unwrap();
    assert_eq!((e.width_mm, e.height_mm), (0, 0));
}

#[test]
fn a_damaged_edid_names_no_mode() {
    let mut b = edid(1920, 1080, 294, 165, (29, 17), false);
    b[60] ^= 1;
    assert_eq!(parse_edid(&b), None, "checksum");
    let mut b = edid(1920, 1080, 294, 165, (29, 17), false);
    b[0] = 0x01;
    assert_eq!(parse_edid(&b), None, "header");
    assert_eq!(parse_edid(&b[..100]), None, "short");
    assert_eq!(parse_edid(&[0u8; 128]), None);
}

#[test]
fn the_physical_size_packs_into_the_handoff_word() {
    assert_eq!(pack_mm(294, 165), 294 | (165 << 16));
    assert_eq!(pack_mm(0, 165), 0);
    assert_eq!(pack_mm(70000, 165), 0);
}
