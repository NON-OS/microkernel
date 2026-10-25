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

/* The registry's and the anchor's calls landed: releaseOf's four words
 * (releaseHash, bondId, publishedAt, withdrawn), anchoredAt's time, and
 * the Bond's credit, read strictly. */

use super::fixture::n;
use super::kinds::every;
use crate::intent::landed;

#[test]
fn a_release_lands_published_or_withdrawn() {
    let e = every();
    let yes = |i: usize, w: &[[u8; 32]]| assert!(landed(&e[i], w), "{:?} on {w:?}", e[i]);
    let no = |i: usize, w: &[[u8; 32]]| assert!(!landed(&e[i], w), "{:?} on {w:?}", e[i]);
    yes(5, &[[6; 32], [2; 32], n(77), n(0)]);
    no(5, &[[6; 32], [2; 32], n(0), n(0)]);
    no(5, &[[6; 32], [2; 32], n(77), n(1)]);
    no(5, &[[7; 32], [2; 32], n(77), n(0)]);
    no(5, &[[6; 32], [3; 32], n(77), n(0)]);
    yes(6, &[[6; 32], [2; 32], n(77), n(1)]);
    no(6, &[[6; 32], [2; 32], n(77), n(0)]);
    no(6, &[[6; 32], [2; 32], n(77), n(2)]);
    yes(7, &[n(1)]);
    no(7, &[n(0)]);
    no(7, &[[0xFF; 32]]);
    yes(8, &[n(0)]);
    no(8, &[n(1)]);
    no(8, &[]);
}
