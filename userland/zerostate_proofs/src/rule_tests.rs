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

//! What the store write may change: anything once keeping is on, and only
//! an erase before.

use crate::keep_rule::{erases_only, may_write};
use crate::sweep::Rng;

const OLD: [u8; 8] = [9, 0, 7, 7, 0, 1, 2, 3];

#[test]
fn once_keeping_is_on_any_write_passes() {
    assert!(may_write(true, &OLD, &[5; 8]));
}

#[test]
fn before_it_only_an_erase_passes() {
    assert!(may_write(false, &OLD, &[0; 8]), "all zeros");
    assert!(may_write(false, &OLD, &[9, 0, 0, 7, 0, 0, 2, 3]), "some bytes cleared");
    assert!(may_write(false, &OLD, &OLD), "nothing changed");
    assert!(!may_write(false, &OLD, &[9, 0, 7, 7, 0, 1, 2, 4]), "one byte rewritten");
    assert!(!may_write(false, &OLD, &[9, 1, 7, 7, 0, 1, 2, 3]), "a zero made non-zero");
    assert!(!may_write(false, &OLD, &[0; 9]), "past what was read");
    assert!(!may_write(false, &OLD, &[0; 7]), "short of it");
}

/* Random old and new sectors: a write passes before keeping exactly when
 * each byte it changes becomes zero. */
#[test]
fn random_writes_pass_unkept_exactly_when_every_change_is_a_zero() {
    let mut r = Rng::new(0x5eed_0006);
    let (mut erases, mut refused) = (0, 0);
    for _ in 0..100_000 {
        let mut old = [0u8; 16];
        r.fill(&mut old);
        let mut new = old;
        for _ in 0..r.below(3) {
            let at = r.below(16) as usize;
            new[at] = if r.below(2) == 0 { 0 } else { r.word() as u8 };
        }
        let want = old.iter().zip(&new).all(|(o, n)| n == o || *n == 0);
        assert_eq!(may_write(false, &old, &new), want, "{old:?} {new:?}");
        assert_eq!(erases_only(&old, &new), want);
        erases += want as u32;
        refused += !want as u32;
    }
    assert!(erases > 10_000 && refused > 10_000, "{erases} {refused}");
}
