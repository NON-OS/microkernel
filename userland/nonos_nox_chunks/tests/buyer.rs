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
use nonos_nox_chunks::{buyer_terms, Buyer, OrderWrong, Sale};
use nonos_nox_config::{DELIVER_MARGIN, MAX_DELIVERY, ORDER_MIN_LEAD};
use sale_fixture::{buyer, sale, NOW, NOX};

#[test]
fn the_deadline_window_and_empty_orders() {
    let t = |s: Sale, b: Buyer| buyer_terms(&K, &s, &NOX, &b, NOW).map(|_| ());
    assert_eq!(t(sale(), buyer(NOW + ORDER_MIN_LEAD - 1)), Err(OrderWrong::Deadline));
    assert_eq!(t(sale(), buyer(NOW + MAX_DELIVERY - DELIVER_MARGIN)), Ok(()));
    assert_eq!(
        t(sale(), buyer(NOW + MAX_DELIVERY - DELIVER_MARGIN + 1)),
        Err(OrderWrong::Deadline)
    );
    assert_eq!(
        t(Sale { amount: 0, ..sale() }, buyer(NOW + ORDER_MIN_LEAD)),
        Err(OrderWrong::Empty)
    );
    assert_eq!(
        t(Sale { content_len: 0, ..sale() }, buyer(NOW + ORDER_MIN_LEAD)),
        Err(OrderWrong::Empty)
    );
    let nobody = Buyer { refund_to: [0; 20], ..buyer(NOW + ORDER_MIN_LEAD) };
    assert_eq!(t(sale(), nobody), Err(OrderWrong::Empty));
}

#[test]
fn a_buyers_salts_never_show_in_its_debug() {
    let b = Buyer { refund_salt: [0xab; 32], salt: [0xcd; 32], ..buyer(NOW + ORDER_MIN_LEAD) };
    let shown = format!("{b:?}");
    assert!(!shown.contains("171") && !shown.contains("205"), "{shown}");
    assert!(shown.contains("refund_salt: \"redacted\"") && shown.contains("deadline"));
}
