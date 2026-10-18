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

//! A voucher channel's form as typed: a deposit in whole NOX, from 1, days
//! from 1 to 365, from a clock that was read, and the listing's cap, the
//! deposit unless typed.

use crate::order::voucher_form::VoucherForm;

const NOX: u128 = 1_000_000_000_000_000_000;
const NOW: u64 = 1_800_000_000;

fn form(deposit: &str, days: &str) -> VoucherForm {
    VoucherForm { deposit: deposit.into(), days: days.into(), ..Default::default() }
}

#[test]
fn a_deposit_and_days_read_as_wei_and_an_expiry() {
    assert_eq!(form("25", "30").read(NOW), Ok((25 * NOX, NOW + 30 * 86_400, 25 * NOX)));
    assert_eq!(form("1", "365").read(NOW), Ok((NOX, NOW + 365 * 86_400, NOX)));
    let capped = VoucherForm { cap: "5".into(), ..form("25", "30") };
    assert_eq!(capped.read(NOW).map(|r| r.2), Ok(5 * NOX), "a cap typed is the cap");
}

#[test]
fn anything_else_is_refused_in_words() {
    for (d, n) in [("0", "1"), ("", "1"), ("1x", "1"), ("1", "0"), ("1", "366"), ("1", "")] {
        assert!(form(d, n).read(NOW).is_err(), "{d} {n}");
    }
    assert!(form("1000000000001", "1").read(NOW).is_err(), "past the most");
    let no_cap = VoucherForm { cap: "0".into(), ..form("1", "1") };
    assert!(matches!(no_cap.read(NOW), Err(s) if s.contains("cap")));
    let unread = form("1", "1").read(0);
    assert!(matches!(unread, Err(s) if s.contains("clock")));
}
