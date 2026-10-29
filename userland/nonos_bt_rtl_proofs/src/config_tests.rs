// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The config: linux-firmware's is taken; one with another signature, a
//! length that is not its own, or an entry that overruns it is not, and
//! no fragment goes out for it.

use nonos_bt_rtl::config::check;
use nonos_bt_rtl::{prepare, RtlError};

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, rom, HP};

#[test]
fn linux_firmwares_config_is_taken() {
    assert_eq!(check(CONFIG), Ok(()));
    assert_eq!(CONFIG[..4], 0x8723_ab55u32.to_le_bytes());
}

#[test]
fn a_malformed_config_is_refused() {
    let mut sig = CONFIG.to_vec();
    sig[0] ^= 1;
    let mut long = CONFIG.to_vec();
    long.push(0);
    let mut overrun = CONFIG.to_vec();
    overrun[8] = 2; // the one entry claims two octets and has one
    let mut field = CONFIG.to_vec();
    field[4] = 5; // the header's length, though the entries fill the file
    for bad in [&[][..], &CONFIG[..5], &sig, &CONFIG[..9], &long, &overrun, &field] {
        assert_eq!(check(bad), Err(RtlError::Config), "{bad:x?}");
    }
}

#[test]
fn a_malformed_config_sends_no_fragment() {
    let mut h = rom(1);
    assert_eq!(prepare(&mut h, HP, FW, &CONFIG[..9]), Err(RtlError::Config));
    assert_eq!(chip(&mut h).opcodes(), [0x1001, 0xfc6d]);
}
