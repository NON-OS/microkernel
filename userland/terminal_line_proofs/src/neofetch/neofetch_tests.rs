// NONOS Operating System (AGPL-3.0-or-later)
//! What `neofetch` prints: figures rounded as people read them, `unknown` for
//! what the kernel did not report, columns that line up, and no escape code
//! in what a redirect writes.

use super::compose::two_column;
use super::facts::{load, memory};
use super::emblem::emblem;
use super::emblem_shape::EMBLEM_W;
use super::palette::palette;

fn s(b: Vec<u8>) -> String {
    String::from_utf8(b).unwrap()
}

#[test]
fn memory_is_used_over_total_in_gib_with_a_share() {
    assert_eq!(s(memory(Some(1_960_000), Some(7_323_000))), "1.9 GiB / 7.0 GiB (27%)");
    assert_eq!(s(memory(Some(0), Some(1_048_576))), "0.0 GiB / 1.0 GiB (0%)");
    assert_eq!(s(memory(None, Some(1_048_576))), "unknown");
    assert_eq!(s(memory(Some(5), None)), "unknown");
    assert_eq!(s(memory(Some(5), Some(0))), "unknown", "a zero total is no figure");
}

#[test]
fn load_reads_the_scheduler_fixed_point() {
    assert_eq!(s(load(Some([2048, 1024, 0]))), "1.00 0.50 0.00");
    assert_eq!(s(load(Some([860, 4096 + 205, 20]))), "0.42 2.10 0.01");
    assert_eq!(s(load(None)), "unknown");
}

#[test]
fn every_row_starts_its_info_in_the_same_column() {
    let right: Vec<(Vec<u8>, Vec<u8>)> =
        (0..10).map(|i| (format!("row{i}").into_bytes(), format!("\x1b[1mrow{i}").into_bytes())).collect();
    let rows = two_column(&emblem(0x07090B), &right, 2);
    assert_eq!(rows.len(), 15);
    for (plain, _) in &rows {
        let text = String::from_utf8(plain.clone()).unwrap();
        let at = text.chars().position(|c| c == 'r');
        assert!(at.is_none() || at == Some(EMBLEM_W + 2), "{text:?}");
    }
}

#[test]
fn a_file_gets_no_escape_codes() {
    let right = vec![(b"os  NONOS".to_vec(), b"\x1b[1;36mos\x1b[0m  NONOS".to_vec())];
    for (plain, _) in two_column(&emblem(0x07090B), &right, 2) {
        assert!(!plain.contains(&0x1b), "{plain:?}");
    }
    for strip in 0..2 {
        let (plain, styled) = palette(strip);
        assert!(!plain.contains(&0x1b));
        assert_eq!(String::from_utf8(plain).unwrap().chars().count(), 16, "eight blocks");
        assert!(styled.ends_with(b"\x1b[0m"), "the colour does not run on");
    }
    assert_ne!(palette(0).1, palette(1).1, "the second strip is the bright one");
}
