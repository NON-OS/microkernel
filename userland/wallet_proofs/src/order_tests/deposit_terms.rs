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

//! The wallet's terms against the ones the meter's id was taken over: one
//! byte off in any of the fourteen is refused, as is a chain nothing pins.

use super::deposit_fixture::{ids, made_at, AT, K};
use super::fixture::{buy, offer, NOW, REFUND_PAIR};
use crate::order::deposit_check::{deposit_check, pinned, ID_WRONG, UNPINNED};

#[test]
fn an_id_over_any_other_term_is_refused_and_so_is_an_unpinned_chain() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let m = made_at(&n);
    for i in 0..14 {
        let mut theirs = m.terms;
        theirs[i][31] ^= 1;
        let mut bad = m;
        (bad.order_id, bad.deposit) = ids(n.chain_id, &theirs, &AT);
        assert_eq!(deposit_check(&K, &n, &bad, Some(AT)), Err(ID_WRONG), "term {i}");
    }
    assert_eq!(deposit_check(&K, &n, &m, None), Err(UNPINNED));
    assert_eq!((pinned(n.chain_id), pinned(1)), (None, None), "nothing pinned yet");
    assert_eq!(deposit_check(&K, &n, &m, pinned(n.chain_id)), Err(UNPINNED));
}
