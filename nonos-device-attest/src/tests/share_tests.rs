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

//! Format 7: an honest proof shares and its shared bytes verify, against
//! the statement alone; they are refused under another context, altered, or
//! read as the other format; a proof that does not verify is never shared.

use stark_proofs::crypto::stark::field::Fp;

use super::fixture::{fixture, ENTROPY};
use crate::prove::{prove, verify, Proof};
use crate::share::{share, verify_shared};

#[test]
fn an_honest_proof_shares_and_its_shared_form_verifies_only_as_itself() {
    let f = fixture();
    let p = prove(&f.st, &f.w, &ENTROPY).expect("an honest proof");
    let s = share(&f.st, &p).expect("it shares");
    assert_eq!(verify_shared(&f.st, &s), Ok(()));
    let mut other = f.st;
    other.context[0] = Fp::from_u64(43);
    assert!(verify_shared(&other, &s).is_err(), "another context");
    assert!(share(&other, &p).is_err(), "a proof that does not verify is not shared");
    assert!(verify_shared(&f.st, &p.bytes).is_err(), "format 5 read as format 7");
    assert!(verify(&f.st, &Proof { bytes: s.clone() }).is_err(), "format 7 read as format 5");
    for at in [0, 40, s.len() / 2, s.len() - 1] {
        let mut bad = s.clone();
        bad[at] ^= 1;
        assert!(verify_shared(&f.st, &bad).is_err(), "byte {at} altered");
    }
    assert!(verify_shared(&f.st, &s[..s.len() - 1]).is_err(), "torn");
}
