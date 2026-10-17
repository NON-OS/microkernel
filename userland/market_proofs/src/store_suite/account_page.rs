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

//! ACCOUNT read page by page: a meter error, or a page that does not read,
//! is no channel at all, and the pages end where the table does.

use nonos_payment_proto::{Account, AccountRow, EAGAIN, EPERM, READY};

use super::held_fixture::{row, NOW};
use crate::store::channel_held::page;

pub fn reply(total: u8, rows: Vec<AccountRow>) -> Vec<u8> {
    let (cap, spent, expiry, left) = (100, 0, NOW, 100);
    let a = Account { flags: READY, address: [9; 20], cap, spent, expiry, left, total, rows };
    a.encode().expect("a page encodes")
}

#[test]
fn a_meter_error_or_a_page_that_does_not_read_is_no_channel() {
    for e in [EAGAIN, EPERM] {
        assert_eq!(page(Err(e), 0), None, "{e}");
    }
    let good = reply(1, Vec::from([row()]));
    assert_eq!(page(Ok(good[..good.len() - 1].to_vec()), 0), None, "short");
    assert_eq!(page(Ok([&good[..], &[0]].concat()), 0), None, "trailing");
    let mut bad = good.clone();
    bad[good.len() - 21] = 2;
    assert_eq!(page(Ok(bad), 0), None, "frozen neither 0 nor 1");
    assert_eq!(page(Ok(Vec::new()), 0), None);
}

#[test]
fn pages_follow_on_until_the_table_ends() {
    let rows = |n| (0..n).map(|_| row()).collect::<Vec<_>>();
    assert_eq!(page(Ok(reply(40, rows(32))), 0).map(|p| p.1), Some(Some(32)));
    assert_eq!(page(Ok(reply(40, rows(8))), 32).map(|p| p.1), Some(None));
    assert_eq!(page(Ok(reply(40, rows(0))), 8).map(|p| p.1), Some(None), "an empty page ends");
}
