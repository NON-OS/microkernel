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

//! The device must hold the enrolled secret, as field words, on slots the
//! kernel's record gives; each lack is refused with its own reason.

use crate::assemble::error::Refusal;
use crate::fixture::{device, field_p, secret_of, SECRET};

#[test]
fn a_secret_that_is_not_the_enrolled_one_is_refused() {
    let mut d = device();
    d.secret = secret_of([1, 2, 3, 4]);
    assert_eq!(d.assemble().err(), Some(Refusal::CommitmentMismatch));
    d.secret = secret_of([1000, 7, 8, 9]);
    assert_eq!(d.assemble().err(), Some(Refusal::CommitmentMismatch), "a decoy's secret");
}

#[test]
fn a_secret_word_at_or_above_p_is_refused() {
    for word in 0..4 {
        for v in [field_p(), field_p() + 7, u64::MAX] {
            let mut words = SECRET;
            words[word] = v;
            let mut d = device();
            d.secret = secret_of(words);
            assert_eq!(d.assemble().err(), Some(Refusal::SecretWord), "word {word}: {v:#x}");
        }
    }
}

#[test]
fn a_slots_record_that_does_not_parse_is_refused() {
    let r = device().record;
    let swapped = [&r[..8], &r[388..], &r[8..388]].concat();
    for bad in [r[..767].to_vec(), [r.clone(), vec![0]].concat(), swapped, Vec::new()] {
        let mut d = device();
        d.record = bad;
        assert_eq!(d.assemble().err(), Some(Refusal::Slots));
    }
    for i in [0, 4, 8, 9, 10, 120, 388, 389, 500, 766] {
        let mut d = device();
        d.record[i] ^= 0x01;
        assert_eq!(d.assemble().err(), Some(Refusal::Slots), "byte {i}");
    }
}
