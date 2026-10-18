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

#[path = "common/k.rs"]
mod k;
#[path = "common/sale_fixture.rs"]
mod sale_fixture;

use k::K;
use nonos_nox_chunks::{buyer_terms, refund_commit, reveal_ok, Mine};
use nonos_nox_config::{DELIVER_MARGIN, DISPUTE_WINDOW, MAX_DELIVERY, ORDER_MIN_LEAD};
use sale_fixture::{buyer, sale, CR, KEY, NOW, NOX};

#[test]
fn what_the_buyer_writes_the_seller_delivers_for() {
    let t = buyer_terms(&K, &sale(), &NOX, &buyer(NOW + ORDER_MIN_LEAD), NOW).unwrap();
    assert_eq!((t.token, t.dispute_window, t.salt), (NOX, DISPUTE_WINDOW, [11; 32]));
    assert_eq!(t.refund_commit, refund_commit(&K, &[9; 20], &[10; 32]));
    let mine = Mine {
        listing_id: [1; 32],
        release_id: [2; 32],
        chunk_root: [3; 32],
        content_len: 3000,
        price: 1_000,
        payee: [4; 20],
        bond_id: [5; 32],
        nox: NOX,
    };
    assert_eq!(reveal_ok(&K, &t, &mine, &KEY, &CR, NOW), Ok(()));
    let late = NOW + MAX_DELIVERY - DELIVER_MARGIN;
    let t = buyer_terms(&K, &sale(), &NOX, &buyer(late), NOW).unwrap();
    assert_eq!(
        reveal_ok(&K, &t, &mine, &KEY, &CR, NOW + DELIVER_MARGIN),
        Ok(()),
        "an hour on, still inside"
    );
}
