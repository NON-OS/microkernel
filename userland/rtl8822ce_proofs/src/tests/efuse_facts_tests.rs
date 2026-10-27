// NONOS Operating System (AGPL-3.0-or-later)
//! The facts come from `struct rtw8822c_efuse`'s offsets and the `PPG_*`
//! bytes of the protected area; a front end never burned is no board; and
//! the station address the map carries plays no part.

use crate::efuse::facts::facts;

fn maps() -> ([u8; 512], [u8; 768]) {
    let mut phys = [0xFF; 512];
    let mut log = [0xFF; 768];
    log[0xCA] = 0x05; // rfe_option
    log[0xB9] = 0xA5; // xtal_k: the cap is the low seven bits
    for (i, b) in log[0x10..0x10 + 12].iter_mut().enumerate() {
        *b = 0x40 + i as u8; // path A: cck_base[6], bw40_base[5], ht_1s_diff
    }
    for (i, b) in log[0x3A..0x3A + 12].iter_mut().enumerate() {
        *b = 0x60 + i as u8; // path B, 42 bytes on
    }
    for (a, v) in [(0x1D4, 0x21), (0x1EE, 0x43), (0x1D2, 0x65), (0x1EF, 0x0B), (0x1B0, 0x0C)] {
        phys[a] = v;
    }
    for (a, v) in [(0x1D6, 0x07), (0x1D5, 0x08), (0x1D8, 0x09), (0x1D7, 0x0A), (0x1EC, 0x11)] {
        phys[a] = v;
    }
    phys[0x1DB] = 0x1F;
    (phys, log)
}

#[test]
fn the_facts_are_at_rtw88s_offsets() {
    let (phys, log) = maps();
    let (rfe, xtal, power, trim) = facts(&phys, &log).expect("a board");
    assert_eq!((rfe, xtal), (0x05, 0x25));
    assert_eq!(power[0].cck, [0x40, 0x41, 0x42, 0x43, 0x44, 0x45]);
    assert_eq!(power[0].bw40, [0x46, 0x47, 0x48, 0x49, 0x4A]);
    assert_eq!(power[0].diff, 0x4B);
    assert_eq!((power[1].cck[0], power[1].bw40[4], power[1].diff), (0x60, 0x6A, 0x6B));
    assert_eq!(trim.tx_2g, [0x21, 0x43, 0x65]);
    assert_eq!(trim.thermal, [0x0B, 0x0C]);
    assert_eq!((trim.pa_bias_2g, trim.pa_bias_5g), ([0x07, 0x08], [0x09, 0x0A]));
    assert_eq!(trim.tx_5g[0], [0x11, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(trim.tx_5g[1], [0xFF, 0xFF, 0xFF, 0xFF, 0x1F]);
}

#[test]
fn a_front_end_never_burned_is_no_board() {
    let (phys, mut log) = maps();
    log[0xCA] = 0xFF;
    assert!(facts(&phys, &log).is_none());
}

#[test]
fn the_whole_front_end_byte_is_taken() {
    let (phys, mut log) = maps();
    log[0xCA] = 0x25;
    assert_eq!(facts(&phys, &log).map(|f| f.0), Some(0x25));
}

#[test]
fn the_station_address_plays_no_part() {
    let (phys, mut log) = maps();
    let before = facts(&phys, &log);
    log[0x120..0x126].copy_from_slice(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    assert_eq!(facts(&phys, &log), before);
}
