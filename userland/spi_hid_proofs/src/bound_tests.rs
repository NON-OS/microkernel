// NONOS Operating System (AGPL-3.0-or-later)
//! The bounds: a header naming a body no transfer carries is no report,
//! refused before a frame is made for it; and a device that never comes
//! up is tried BRING_UP_TRIES times, then given up on with EXIT_GAVE_UP.

use nonos_spi_hid::{header, BODY_MAX, FRAME_HEAD, HEADER_SYNC, TRANSFER_MAX};

use crate::retry::{Tries, BRING_UP_TRIES};

const RUN: &str = include_str!("../../capsule_driver_spi_hid/src/run.rs");
const WIRE: &str = include_str!("../../capsule_driver_spi_hid/src/spi/wire.rs");

fn hdr(words: u32) -> [u8; 4] {
    (3 | words << 8 | 1 << 22 | (HEADER_SYNC as u32) << 24).to_le_bytes()
}

#[test]
fn a_body_longer_than_a_transfer_is_no_report() {
    assert_eq!((TRANSFER_MAX, BODY_MAX + FRAME_HEAD), (4000, 4000));
    assert!(WIRE.contains("pub const TRANSFER_MAX: usize = 4000;"), "spi0's bound");
    let fits = (BODY_MAX / 4) as u32;
    assert_eq!(header(hdr(fits)).map(|h| h.body_len), Some(BODY_MAX));
    assert_eq!(header(hdr(fits + 1)), None, "a word past the transfer");
    assert_eq!(header(hdr(0x3FFF)), None, "the field's most, 64 KiB");
}

#[test]
fn a_device_that_never_answers_is_given_up_on() {
    let mut t = Tries::default();
    for _ in 1..BRING_UP_TRIES {
        assert!(!t.failed(), "given up early");
    }
    assert!(t.failed(), "the last try");
    assert_eq!(BRING_UP_TRIES, 20, "ten seconds at 500 ms");
    let at = |s: &str| RUN.find(s).unwrap_or_else(|| panic!("{s} not in run.rs"));
    assert!(at("if tries.failed() {") < at("mk_exit(EXIT_GAVE_UP);"));
    assert!(at("mk_exit(EXIT_GAVE_UP);") < at("let _ = mk_idle_ms(RETRY_MS);"));
    assert!(at("let _ = mk_idle_ms(RETRY_MS);") < at("\n            tries = Tries::default();"));
}
