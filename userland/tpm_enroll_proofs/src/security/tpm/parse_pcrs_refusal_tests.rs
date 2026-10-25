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

//! What the PCR read refuses in the answer's frame: a sessions tag on a
//! command sent without one, more than one selection, a bank other than
//! SHA-256, a select of other than three bytes; and a round that reads
//! nothing, refused at once, since the bank is not allocated.

use crate::security::tpm::boot_reads::pcrs::read_with;
use crate::security::tpm::boot_reads::pcrs_round::parse;

use super::parse_pcrs_tests::answer;

#[test]
fn an_answer_whose_frame_is_not_a_plain_read_s_is_refused() {
    let good = answer([0x01, 0, 0], &[[1; 32]], &[]);
    assert!(parse(&good, 0x01).is_ok());
    for (at, value, what) in [(1, 0x02, "sessions tag"), (17, 2, "two selections")] {
        let mut b = good.clone();
        b[at] = value;
        assert!(parse(&b, 0x01).is_err(), "{what}");
    }
    for (at, value, what) in [(19, 0x04, "SHA-1 bank"), (20, 2, "two select bytes")] {
        let mut b = good.clone();
        b[at] = value;
        assert!(parse(&b, 0x01).is_err(), "{what}");
    }
}

#[test]
fn a_round_that_reads_nothing_is_refused_at_once() {
    let mut rounds = 0;
    let r = read_with(&[0], |_| {
        rounds += 1;
        Ok(vec![])
    });
    assert!(r.is_err());
    assert_eq!(rounds, 1, "not asked again");
}
