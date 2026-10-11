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

use nonos_proof_abi::{allows, hist_commit, syscall_root};

use crate::fixture::family;

#[test]
fn the_fixtures_words_follow_from_its_histogram_and_set() {
    let f = family();
    let mut set = [0u8; 65];
    set.copy_from_slice(&f.policy[152..217]);
    let ok = |b: usize| u64::from(f.hist[2 * b]);
    let err = |b: usize| u64::from(f.hist[2 * b + 1]);
    let n: u64 = (0..513).map(|b| ok(b) + err(b)).sum();
    let u: u64 = (0..513).filter(|&b| !allows(&set, b)).map(err).sum();
    assert!((0..513).all(|b| allows(&set, b) || ok(b) == 0), "nothing outside A succeeded");
    assert_eq!((f.publics[8], f.publics[9]), (n, u));
    assert_eq!(hist_commit(&f.hist, &f.salt), Some(f.record.hc));
    assert_eq!(syscall_root(&set).map(|a| a.to_vec()), Some(f.publics[4..8].to_vec()));
}
