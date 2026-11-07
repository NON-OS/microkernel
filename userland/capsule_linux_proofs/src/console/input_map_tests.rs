// NONOS Operating System (AGPL-3.0-or-later)
//! What the NONOS terminal sends is mapped as c_iflag says before a
//! program reads it, as Linux's n_tty maps it: SimpleX's terminal client,
//! raw with ICRNL kept, waited for a newline its Enter never made.

use super::input_map_check::{check_bytes, check_map};
use super::tty_rules::map_input;
use crate::linux::console::termios::COOKED;

const ISTRIP: u32 = 0o40;
const INLCR: u32 = 0o100;
const IGNCR: u32 = 0o200;
const ICRNL: u32 = 0o400;

fn with_iflag(i: u32) -> [u8; 36] {
    let mut t = COOKED;
    t[..4].copy_from_slice(&i.to_le_bytes());
    t
}

fn mapped(t: &[u8], typed: &[u8]) -> Vec<u8> {
    let mut got = typed.to_vec();
    let kept = map_input(t, &mut got);
    got.truncate(kept);
    got
}

#[test]
fn enter_is_a_newline_under_icrnl_as_the_cooked_settings_have() {
    assert_eq!(mapped(&COOKED, b"/ad\r"), b"/ad\n");
    assert_eq!(mapped(&with_iflag(ICRNL), b"a\rb\r"), b"a\nb\n");
}

#[test]
fn without_icrnl_a_carriage_return_stays_one() {
    assert_eq!(mapped(&with_iflag(0), b"x\r\n"), b"x\r\n");
}

#[test]
fn igncr_drops_it_and_inlcr_turns_a_newline_back() {
    assert_eq!(mapped(&with_iflag(IGNCR | ICRNL), b"a\rb\r\n"), b"ab\n");
    assert_eq!(mapped(&with_iflag(INLCR), b"a\nb"), b"a\rb");
    assert_eq!(mapped(&with_iflag(ICRNL | INLCR), b"\r\n"), b"\n\r", "each byte once");
}

#[test]
fn istrip_takes_the_eighth_bit_before_the_rest() {
    assert_eq!(mapped(&with_iflag(ISTRIP), &[0xC1, 0x8D]), [0x41, 0x0D]);
    assert_eq!(mapped(&with_iflag(ISTRIP | ICRNL), &[0x8D]), b"\n");
    assert_eq!(mapped(&with_iflag(0), &[0xC3, 0xA9]), [0xC3, 0xA9], "UTF-8 kept whole");
}

#[test]
fn every_other_byte_and_short_settings_pass_as_typed() {
    let all: Vec<u8> = (0..=255u8).filter(|b| *b != b'\r' && *b != b'\n').collect();
    assert_eq!(mapped(&COOKED, &all), all);
    assert_eq!(mapped(&[], b"a\rb"), b"a\rb", "no c_iflag: nothing mapped");
    assert_eq!(mapped(&COOKED, b""), b"");
}

#[test]
fn the_console_maps_what_it_pulls_before_it_is_queued() {
    const INPUT: &str = include_str!("../../../capsule_linux/src/linux/console/input.rs");
    assert!(INPUT.contains(
        "let kept = map_input(&c.termios, got);\n            c.queue.push(&got[..kept]);"
    ));
}

#[test]
fn every_byte_under_every_mapping_flag_holds_to_the_check() {
    let all: Vec<u8> = (0..=255u8).collect();
    for bits in 0..16u32 {
        let flags = [ISTRIP, INLCR, IGNCR, ICRNL];
        let i = flags
            .iter()
            .enumerate()
            .filter(|(k, _)| bits & (1 << k) != 0)
            .fold(0, |a, (_, f)| a | f);
        check_map(&with_iflag(i), &all);
        check_map(&with_iflag(i), b"\r\n\r\r\n\n\x8d\x8a");
    }
    check_bytes(&[]);
    check_bytes(&[0xff; 40]);
}
