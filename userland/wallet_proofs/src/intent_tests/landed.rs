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

/* An approve or a Bond call has landed when its view says so, each answer
 * read strictly: the exact allowance, the bond held for its key, the agent,
 * an unlockAt, a bond withdrawn. Any other count or shape is not landed. */

use super::fixture::n;
use super::kinds::every;
use crate::intent::landed;

fn bond(payee: [u8; 32], key: [u8; 32], amount: u128, unlock: u128) -> Vec<[u8; 32]> {
    vec![payee, key, n(amount), n(unlock), n(0)]
}

#[test]
fn each_kind_lands_on_its_answer() {
    let e = every();
    let me = n(0x99);
    let yes = |i: usize, w: &[[u8; 32]]| assert!(landed(&e[i], w), "{:?} on {w:?}", e[i]);
    let no = |i: usize, w: &[[u8; 32]]| assert!(!landed(&e[i], w), "{:?} on {w:?}", e[i]);
    yes(0, &[n(5)]);
    no(0, &[n(4)]);
    no(0, &[n(6)]);
    no(0, &[n(5), n(5)]);
    yes(1, &bond(me, [1; 32], 7, 0));
    yes(1, &bond(me, [1; 32], 8, 0));
    no(1, &bond(me, [1; 32], 6, 0));
    no(1, &bond(n(0), [1; 32], 7, 0));
    no(1, &bond(me, [2; 32], 7, 0));
    no(1, &bond([0xFF; 32], [1; 32], 7, 0));
    let mut agent = [0u8; 32];
    agent[12..].copy_from_slice(&[3; 20]);
    yes(2, &[agent]);
    no(2, &[n(3)]);
    agent[0] = 1;
    no(2, &[agent]);
    yes(3, &bond(me, [1; 32], 7, 100));
    no(3, &bond(me, [1; 32], 7, 0));
    no(3, &bond(me, [1; 32], 7, u128::from(u64::MAX) + 1));
    yes(4, &bond(me, [1; 32], 0, 100));
    no(4, &bond(me, [1; 32], 1, 100));
    no(4, &bond(me, [1; 32], 0, 100)[..4]);
}
