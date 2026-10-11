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

//! ACCOUNT and DRAIN round-trip, and every cut or tail is refused.

use nonos_payment_proto::*;

mod common;
use common::refused_cut_and_tail;

#[test]
fn account_and_drain_round_trip() {
    let row = AccountRow {
        channel_id: [9; 32],
        listing_id: [8; 32],
        chain_id: 1,
        deposit: 10,
        settled: 2,
        cumulative: 3,
        expiry: 4,
        frozen: true,
    };
    let a = Account {
        flags: READY | KEPT,
        address: [5; 20],
        cap: 1,
        spent: 2,
        expiry: 3,
        left: 4,
        total: 1,
        rows: vec![row],
    };
    let b = a.encode();
    assert_eq!((b.len(), ACCOUNT_ROW_LEN), (79 + 129, 129));
    assert_eq!(Account::parse(&b), Some(a));
    refused_cut_and_tail(&b, Account::parse);
    let mut bad = b.clone();
    *bad.last_mut().unwrap() = 2;
    assert_eq!(Account::parse(&bad), None);
    let v = VoucherRecord {
        channel_id: [1; 32],
        payer: [2; 20],
        payee: [3; 20],
        listing_id: [4; 32],
        unit: [5; 32],
        cumulative: 6,
        seq: 7,
        expiry: 8,
        usage_root: [9; 32],
        sig: [10; 65],
    };
    let d = Drain { total: 3, rows: vec![v, v] };
    let b = d.encode();
    assert_eq!(b.len(), 2 + 2 * VOUCHER_RECORD_LEN);
    assert_eq!(Drain::parse(&b), Some(d));
    refused_cut_and_tail(&b, Drain::parse);
    assert_eq!(VoucherRecord::parse(&b[2..2 + VOUCHER_RECORD_LEN]), Some(v));
}
