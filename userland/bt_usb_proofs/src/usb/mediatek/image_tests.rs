/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The patch file's header and section maps: each section's spec, offset
//! and length read where the layout puts them, and a file whose header or
//! maps point outside it refused whole.

use super::build::{image, octets, three, Part};
use super::image::sections;

#[test]
fn each_section_is_read_from_its_map() {
    let img = image(&three());
    let s = sections(&img).expect("parses");
    assert_eq!(s.len(), 3);
    assert_eq!((s[0].at, s[0].len), (96 + 3 * 64, 700));
    assert_eq!((s[1].at, s[1].len), (96 + 3 * 64 + 700, 250));
    assert_eq!(&img[s[1].at..s[1].at + 250], &octets(250, 2)[..]);
    assert_eq!(s[0].spec[..8], [0, 0, 0x90, 0, 0xbc, 2, 0, 0], "load address, then length");
    assert_eq!(s[1].spec[..4], 0x0091_8000u32.to_le_bytes());
    assert_eq!(s[2].len, 0, "a section with nothing to download");
}

#[test]
fn a_header_or_map_cut_short_is_refused() {
    let empty: Vec<Part> = (0..3).map(|addr| Part { addr, data: Vec::new() }).collect();
    let img = image(&empty);
    assert_eq!(img.len(), 96 + 3 * 64);
    assert_eq!(sections(&img).map(|s| s.len()), Some(3));
    assert!(sections(&img[..95]).is_none(), "no global descriptor");
    assert!(sections(&img[..img.len() - 1]).is_none(), "the third map cut");
    let mut more = img.clone();
    more[32 + 12] = 4;
    assert!(sections(&more).is_none(), "a count past the maps");
}

#[test]
fn a_section_past_the_end_of_its_file_is_refused() {
    let img = image(&three());
    assert!(sections(&img[..img.len() - 1]).is_none(), "the last octet gone");
    let mut wild = img.clone();
    wild[96 + 4..96 + 8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(sections(&wild).is_none(), "an offset that wraps");
    let mut long = img.clone();
    long[96 + 64 + 16] = 0xff;
    assert!(sections(&long).is_none(), "a length past the file");
}

#[test]
fn an_empty_section_is_not_held_to_its_offset() {
    let mut img = image(&three());
    img[96 + 128 + 4..96 + 128 + 8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(sections(&img).map(|s| s.len()), Some(3));
}

#[test]
fn a_count_past_any_patch_is_refused() {
    let parts: Vec<Part> = (0..17).map(|i| Part { addr: i, data: vec![1] }).collect();
    assert!(sections(&image(&parts)).is_none(), "seventeen sections");
    assert_eq!(sections(&image(&parts[..16])).map(|s| s.len()), Some(16));
}
