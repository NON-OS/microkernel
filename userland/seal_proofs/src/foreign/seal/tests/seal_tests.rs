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

//! A family sealed through finish.rs: the record, the witness and the list
//! the prover reads back.

use nonos_proof_abi::{hist_commit, meas_chain, HIST_WORDS};

use super::fixture::{family, SALT};
use crate::foreign::seal::finish::finish;
use crate::foreign::seal::hist::slot;
use crate::foreign::seal::sealed::WITNESS_BYTES;
use crate::foreign::seal::table::{LIVE, SEALED};
use crate::foreign::trap_table::Answer;

fn witness_of(w: &[u8]) -> ([u32; HIST_WORDS], [u64; 4], [u8; 32]) {
    let word = |at: usize| u64::from_le_bytes(w[at..at + 8].try_into().unwrap());
    let h = core::array::from_fn(|i| u32::from_le_bytes(w[4 * i..4 * i + 4].try_into().unwrap()));
    let at = 4 * HIST_WORDS;
    (h, core::array::from_fn(|i| word(at + 8 * i)), w[at + 32..].try_into().unwrap())
}

#[test]
fn a_sealed_family_reads_back_whole() {
    let mut f = family(77);
    (f.app_id, f.diag) = (Some([0xA1; 32]), [0xD1; 32]);
    f.measured(&[0xC3; 32]);
    f.trapped(5, 0);
    f.answered(5, 0, Answer::Value(4));
    f.trapped(6, 101); // never answered: sealed as died, a success
    LIVE.lock().push(f);
    let n = finish(77).expect("a seal");
    assert!(LIVE.lock().iter().all(|f| f.sup != 77), "a sealed family is no longer live");
    let sealed = SEALED.lock();
    let s = sealed.iter().find(|s| s.seal_no == n).expect("kept");
    let r = &s.record;
    let hc: Vec<u64> =
        (0..4).map(|i| u64::from_le_bytes(r[104 + 8 * i..][..8].try_into().unwrap())).collect();
    let all = s.bytes();
    assert_eq!((all.len(), &all[..240]), (s.bytes_len(), &r[..]), "record first");
    assert_eq!(&all[240 + 4 * HIST_WORDS + 64..], &[0xC3; 32][..], "the list last");
    let (h, salt, diag) = witness_of(&all[240..240 + WITNESS_BYTES]);
    assert_eq!(
        (&r[6..8], &r[56..64], &r[176..208]),
        (&[1, 0][..], &n.to_le_bytes()[..], &[0xA1; 32][..])
    );
    assert_eq!((h[slot(0, false)], h[slot(101, false)], salt, diag), (1, 1, SALT, [0xD1; 32]));
    assert_eq!(hist_commit(&h, &salt).map(|w| w.to_vec()), Some(hc), "the witness opens Hc");
    assert_eq!(&r[136..168], &meas_chain(&[[0xC3; 32]])[..]);
    assert!(r[208..].iter().all(|&x| x == 0), "the diag never enters the record");
    drop(sealed);
    assert_eq!(finish(77), None, "nothing left to seal");
}
