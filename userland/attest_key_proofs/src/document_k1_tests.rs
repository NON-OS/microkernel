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

//! the fields: after the DMA posture, the count of quoted PCRs, each
//! index and its value in the kernel's order, then the log, length first.
//! And what came before the posture is where version one put it.

use super::document_tests::{doc, PCRS_AT};
use crate::security::attest_doc::document::{log_fits, MAX_LOG, QUOTED_PCRS};

#[test]
fn the_quoted_pcrs_follow_the_posture_in_order_then_the_log() {
    let b = doc().encode();
    assert_eq!(QUOTED_PCRS, [0, 1, 2, 4, 7, 8, 9, 14]);
    assert_eq!(b[PCRS_AT], 8);
    for (k, index) in QUOTED_PCRS.iter().enumerate() {
        let at = PCRS_AT + 1 + 33 * k;
        assert_eq!(b[at], *index, "index {k}");
        assert_eq!(&b[at + 1..at + 33], &[0x30 + k as u8; 32], "value {k}");
    }
    let log_at = PCRS_AT + 1 + 33 * 8;
    assert_eq!(&b[log_at..log_at + 4], &3u32.to_be_bytes());
    assert_eq!(&b[log_at + 4..log_at + 7], b"log");
    assert_eq!(MAX_LOG, 1_048_576);
}

#[test]
fn an_empty_log_is_carried_as_a_zero_length() {
    let mut d = doc();
    d.log.clear();
    let b = d.encode();
    let log_at = PCRS_AT + 1 + 33 * 8;
    assert_eq!(&b[log_at..log_at + 8], &[0, 0, 0, 0, 0, 0, 0, 4], "then the attest length");
}

#[test]
fn everything_before_the_posture_is_where_version_one_put_it() {
    let b = doc().encode();
    assert_eq!(&b[..8], b"NONOSATT");
    assert_eq!(&b[44..76], &[0xAB; 32]);
    assert_eq!(&b[76..80], &3u32.to_be_bytes());
    assert_eq!(b[80], 1);
}

#[test]
fn a_log_fits_up_to_the_cap_and_not_a_byte_more() {
    assert!(log_fits(&[]) && log_fits(&vec![0; MAX_LOG]));
    assert!(!log_fits(&vec![0; MAX_LOG + 1]));
}
