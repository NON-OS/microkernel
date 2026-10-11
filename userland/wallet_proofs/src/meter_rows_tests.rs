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

//! The meter screen's lines (B8), from ACCOUNT replies built here.

use nonos_payment_proto::{Account, AccountRow, KEPT, READY};

use crate::meter_channel::channel_line;
use crate::meter_rows::{budget_rows, state_line};

const NOW: u64 = 1_800_000_000;

fn nox(wei: u128) -> String {
    format!("{}", wei / 1_000_000_000_000_000_000)
}

fn account(flags: u8, cap: u128, expiry: u64) -> Account {
    let left = cap.saturating_sub(40);
    Account { flags, address: [7; 20], cap, spent: 40, expiry, left, total: 1, rows: vec![] }
}

#[test]
fn the_meter_says_how_it_stands() {
    assert_eq!(state_line(None), "did not answer");
    assert!(state_line(Some(&account(KEPT, 0, 0))).starts_with("no key open"));
    assert_eq!(state_line(Some(&account(READY, 0, 0))), "open for this session only");
    assert_eq!(state_line(Some(&account(READY | KEPT, 0, 0))), "open, kept on this machine");
}

#[test]
fn a_budget_reads_as_figures_and_time_left() {
    let none = budget_rows(&account(READY, 0, 0), NOW, nox);
    assert_eq!(none, vec![("budget", String::from("none set: the meter pays nothing"))]);
    let e18 = 1_000_000_000_000_000_000u128;
    let a =
        Account { spent: 40 * e18, left: 60 * e18, ..account(READY, 100 * e18, NOW + 3 * 86_400) };
    let rows = budget_rows(&a, NOW, nox);
    assert_eq!(rows[0], ("budget", String::from("100 NOX")));
    assert_eq!((rows[1].1.as_str(), rows[2].1.as_str()), ("40 NOX", "60 NOX"));
    assert_eq!(rows[3], ("budget ends", String::from("in 3 days")));
    let soon = budget_rows(&Account { expiry: NOW + 7_200, ..a.clone() }, NOW, nox);
    assert_eq!(soon[3].1, "in 2 hours");
    assert!(budget_rows(&Account { expiry: NOW, ..a }, NOW, nox)[3].1.starts_with("ended"));
}

#[test]
fn a_channel_says_where_what_was_paid_and_whether_it_froze() {
    let e18 = 1_000_000_000_000_000_000u128;
    let r = AccountRow {
        channel_id: [1; 32],
        listing_id: [2; 32],
        chain_id: 11_155_111,
        deposit: 50 * e18,
        settled: 10 * e18,
        cumulative: 12 * e18,
        expiry: NOW,
        frozen: false,
    };
    assert_eq!(channel_line(&r, nox), "Sepolia · 50 held · 10 paid · 12 signed");
    let f = channel_line(&AccountRow { frozen: true, chain_id: 1, ..r }, nox);
    assert!(f.starts_with("mainnet") && f.contains("frozen"));
}
