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

//! The witness a sealed run makes: every call that succeeded is in the set
//! (OUTSIDE), and the record's Hc is the one H makes (PROVER).

use nonos_proof_abi::wire::{OUTSIDE, PROVER};
use nonos_proof_abi::{hist_commit, syscall_root, HIST_WORDS};

use crate::fixture::{hist, policy, record, sealed_bytes, LISTING, SALT};
use crate::market::isolation::witness;
use crate::market::sealed::{split, Sealed};

const ALLOWED: [usize; 4] = [0, 1, 60, 231];

fn sealed(p: &[u8; 256], h: &[u32; HIST_WORDS]) -> Sealed {
    split(&sealed_bytes(&record(p, h, &[]), h, &[])).expect("splits")
}

#[test]
fn an_honest_run_gives_the_records_statement() {
    let (p, h) = (policy(&LISTING, &ALLOWED), hist());
    let (w, st) = witness(&sealed(&p, &h), &p).expect("inside the set");
    assert_eq!((w.hist, w.salt), (h, SALT));
    assert_eq!(Some(st.hc), hist_commit(&h, &SALT));
    assert_eq!(Some(st.a), syscall_root(&w.set));
}

#[test]
fn a_call_outside_the_set_that_succeeded_is_outside() {
    let p = policy(&LISTING, &ALLOWED);
    let mut h = hist();
    h[2 * 101] = 1;
    assert_eq!(witness(&sealed(&p, &h), &p).err(), Some(OUTSIDE));
    h[2 * 101] = 0;
    h[2 * 59 + 1] = 9;
    assert!(witness(&sealed(&p, &h), &p).is_ok(), "a refused call outside is no matter");
}

#[test]
fn an_hc_that_is_not_the_records_is_the_provers_fault() {
    let (p, h) = (policy(&LISTING, &ALLOWED), hist());
    let mut s = sealed(&p, &h);
    s.hist[0] += 1;
    assert_eq!(witness(&s, &p).err(), Some(PROVER), "H is not the sealed one");
    let mut s = sealed(&p, &h);
    s.salt[3] ^= 1;
    assert_eq!(witness(&s, &p).err(), Some(PROVER), "the salt is not the sealed one");
}
