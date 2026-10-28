// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Fragments: numbered 0 to 0x7f then 1 to 0x7f again, 252 octets each
//! but the last, which alone carries the data-end bit and holds what is
//! left, none when the image is a multiple of 252.

use nonos_bt_rtl::fragments::{count, fragments, index, DATA_END, FRAGMENT};
use nonos_bt_rtl::prepare;

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, expected, rom, HP};

#[test]
fn the_numbering_wraps_from_0x7f_to_1() {
    let first: Vec<u8> = (0..=0x7f).map(index).collect();
    assert_eq!(first, (0..=0x7f).collect::<Vec<u8>>());
    assert_eq!((index(0x80), index(0xfe), index(0xff), index(0x17d)), (1, 0x7f, 1, 0x7f));
}

#[test]
fn the_real_image_goes_in_139_numbered_fragments() {
    let mut h = rom(1);
    prepare(&mut h, HP, FW, CONFIG).unwrap();
    let want: Vec<u8> = (0..=0x7f).chain(1..=10).chain([11 | DATA_END]).collect();
    assert_eq!(chip(&mut h).indices(), want);
    assert_eq!(chip(&mut h).broken, None);
}

#[test]
fn only_the_last_fragment_is_short_and_marked() {
    let img = expected(1);
    let all: Vec<(u8, &[u8])> = fragments(&img).collect();
    assert_eq!(all.len(), img.len() / FRAGMENT + 1);
    let (last, rest) = all.split_last().unwrap();
    assert!(rest.iter().all(|(i, d)| i & DATA_END == 0 && d.len() == FRAGMENT));
    assert_eq!((last.0 & DATA_END, last.1.len()), (DATA_END, img.len() % FRAGMENT));
    assert_eq!(all.iter().flat_map(|(_, d)| d.iter().copied()).collect::<Vec<u8>>(), img);
}

#[test]
fn an_image_of_whole_fragments_ends_with_an_empty_last_one() {
    let img = vec![0x5a; 2 * FRAGMENT];
    let all: Vec<(u8, &[u8])> = fragments(&img).collect();
    assert_eq!(all.len(), 3);
    assert_eq!((all[2].0, all[2].1.len()), (2 | DATA_END, 0));
}

#[test]
fn a_short_image_is_one_last_fragment() {
    let img = [7u8; 10];
    let all: Vec<(u8, &[u8])> = fragments(&img).collect();
    assert_eq!(all, [(DATA_END, &img[..])]);
    assert_eq!((count(0), count(251), count(252), count(253)), (1, 1, 2, 2));
}
