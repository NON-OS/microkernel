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

//! MkSealRead's bytes as the job splits them: the record, H, the salt and
//! the list come back whole; a list that is not the record's count, a short
//! witness or a record that does not decode is refused.

use nonos_proof_abi::{HIST_WORDS, RECORD_BYTES};

use crate::fixture::{hist, policy, record, sealed_bytes, LISTING, SALT};
use crate::market::sealed::{split, WITNESS_BYTES};

const ALLOWED: [usize; 4] = [0, 1, 60, 231];

fn meas() -> Vec<[u8; 32]> {
    vec![[0x11; 32], [0x22; 32], [0x33; 32]]
}

#[test]
fn a_sealed_family_splits_into_its_parts() {
    let (p, h) = (policy(&LISTING, &ALLOWED), hist());
    let rec = record(&p, &h, &meas());
    let b = sealed_bytes(&rec, &h, &meas());
    assert_eq!(WITNESS_BYTES, 4 * HIST_WORDS + 64);
    assert_eq!(b.len(), RECORD_BYTES + WITNESS_BYTES + 3 * 32);
    let s = split(&b).expect("splits");
    assert_eq!(s.raw, rec.encode());
    assert_eq!(s.record, rec);
    assert_eq!((s.hist, s.salt, s.list.clone()), (h, SALT, meas()));
}

#[test]
fn a_list_that_is_not_the_records_count_is_refused() {
    let (p, h) = (policy(&LISTING, &ALLOWED), hist());
    let rec = record(&p, &h, &meas());
    let b = sealed_bytes(&rec, &h, &meas());
    assert!(split(&b[..b.len() - 32]).is_none(), "one entry short");
    assert!(split(&b[..b.len() - 1]).is_none(), "a torn entry");
    assert!(split(&[b.clone(), vec![0; 32]].concat()).is_none(), "one entry over");
    let none = record(&p, &h, &[]);
    assert!(split(&sealed_bytes(&none, &h, &[])).is_some(), "an empty list");
}

#[test]
fn a_short_witness_or_a_bad_record_is_refused() {
    let (p, h) = (policy(&LISTING, &ALLOWED), hist());
    let rec = record(&p, &h, &[]);
    let b = sealed_bytes(&rec, &h, &[]);
    for n in [0, RECORD_BYTES - 1, RECORD_BYTES, b.len() - 1] {
        assert!(split(&b[..n]).is_none(), "{n} bytes");
    }
    let mut bad = b.clone();
    bad[..4].copy_from_slice(&[0; 4]);
    assert!(split(&bad).is_none(), "a record that does not decode");
}
