// NONOS Operating System (AGPL-3.0-or-later)
//! A whole read takes the board facts and leaves the factory address behind,
//! and the reply's old BAR and window words are zero.

use super::card::{carries_factory, Efuse};
use crate::efuse::{diag, read};
use crate::link::LinkStats;
use crate::serve_status::{status_body, STATUS_BODY_LEN};

#[test]
fn a_whole_read_reports_the_board_and_never_the_address() {
    let info = read(&Efuse::new(None)).expect("the model reads whole");
    assert_eq!((info.rfe, info.pkg), (2, 1), "the option byte is read");
    let stats = LinkStats { tx_ok: 7, rx_ring: 9, rekeys: 1, ..LinkStats::default() };
    let mut out = [0xEEu8; STATUS_BODY_LEN + 4];
    assert_eq!(status_body(8, &stats, &diag(), &mut out), Some(STATUS_BODY_LEN));
    assert!(!carries_factory(&out[..STATUS_BODY_LEN]));
    assert_eq!(out[0], 8);
    assert_eq!(out[1..5], 7u32.to_le_bytes());
    assert_eq!(out[37..45], [0; 8], "no BAR, no window address");
    assert_eq!(out[49..53], 1u32.to_le_bytes(), "rekeys last");
    assert_eq!(out[STATUS_BODY_LEN..], [0xEE; 4], "nothing past the body");
    assert_eq!(status_body(8, &stats, &diag(), &mut [0u8; STATUS_BODY_LEN - 1]), None);
}
