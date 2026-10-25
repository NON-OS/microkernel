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

//! The PCR read's answers and a quote's coverage, as bytes: what the
//! kernel refuses that a well-behaved TPM never sends. A read of PCRs it
//! was not asked for, a count that is not the selection's, bytes after the
//! last digest, a loop that stops before every PCR is read.

use crate::security::tpm::boot_reads::pcrs::read_with;
use crate::security::tpm::boot_reads::pcrs_round::parse;

/// A `TPM2_PCR_Read` answer: header, counter, one selection, the digests.
pub(super) fn answer(select: [u8; 3], digests: &[[u8; 32]], extra: &[u8]) -> Vec<u8> {
    let mut b = vec![0x80, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];
    b.extend([0, 0, 0, 1, 0x00, 0x0B, 3]);
    b.extend(select);
    b.extend((digests.len() as u32).to_be_bytes());
    for d in digests {
        b.extend([0, 32]);
        b.extend(d);
    }
    b.extend(extra);
    let n = b.len() as u32;
    b[2..6].copy_from_slice(&n.to_be_bytes());
    b
}

#[test]
fn an_answer_names_each_value_by_the_pcr_it_read() {
    let got = parse(&answer([0x11, 0, 0x80], &[[1; 32], [2; 32], [3; 32]], &[]), u32::MAX);
    assert_eq!(got.expect("read"), vec![(0, [1; 32]), (4, [2; 32]), (23, [3; 32])]);
}

#[test]
fn an_answer_beyond_the_question_or_out_of_count_is_refused() {
    assert!(parse(&answer([0x11, 0, 0], &[[1; 32], [2; 32]], &[]), 0x01).is_err(), "unasked");
    assert!(parse(&answer([0x01, 0, 0], &[[1; 32], [2; 32]], &[]), 0x01).is_err(), "two for one");
    assert!(parse(&answer([0x03, 0, 0], &[[1; 32]], &[]), 0x03).is_err(), "one for two");
    assert!(parse(&answer([0x01, 0, 0], &[[1; 32]], &[0]), 0x01).is_err(), "a byte after");
    assert!(parse(&answer([0x01, 0, 0], &[[1; 32]], &[]), 0x01).is_ok());
}

#[test]
fn the_loop_reads_until_done_and_refuses_a_read_that_stalls() {
    let mut rounds = vec![vec![(9, [9; 32])], vec![(0, [1; 32])]].into_iter();
    let got = read_with(&[0, 9], |_| Ok(rounds.next().unwrap_or_default()));
    assert_eq!(got.expect("two rounds"), [[1; 32], [9; 32]]);
    assert!(read_with(&[0], |_| Ok(vec![])).is_err(), "a round that reads nothing");
    assert!(read_with(&[0, 1], |_| Ok(vec![(0, [1; 32]), (0, [1; 32])])).is_err(), "twice");
    let mut once = Some(vec![(0, [1; 32])]);
    let stalls = read_with(&[0, 1], |_| Ok(once.take().unwrap_or_else(|| vec![(0, [1; 32])])));
    assert!(stalls.is_err(), "a PCR read again, the other never");
    assert!(
        read_with(&[40], |_| Ok(vec![])).is_err()
            && read_with(&[4, 4], |_| Ok(vec![(4, [4; 32])])).is_err()
    );
}
