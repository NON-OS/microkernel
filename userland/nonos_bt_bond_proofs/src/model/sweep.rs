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

//! Keep, forget and reopen in seeded orders, a keyring fault at each keep.

use super::kit::{ident, Rng, SEEDS};
use super::run::{check, keep, Model, N};
use crate::faulty::Faulty;
use nonos_bt_bond::Store;

#[test]
fn the_level_never_rises_and_a_forget_is_final() {
    for seed in SEEDS {
        let mut s = Store::open(Faulty::default()).unwrap();
        let mut m: Model = [None; N];
        let mut rng = Rng(seed);
        for _ in 0..600 {
            match rng.below(5) {
                0 | 1 => {
                    let (i, auth, fault) = (rng.below(N), rng.below(2) == 1, rng.below(3));
                    keep(&mut s, &mut m, i, auth, fault);
                }
                2 => {
                    let i = rng.below(N);
                    s.forget(0, &ident(i)).unwrap();
                    m[i] = None;
                }
                _ => {
                    s = Store::open(s.into_keys()).unwrap();
                    check(&s, &m, true);
                    continue;
                }
            }
            check(&s, &m, false);
        }
    }
}
