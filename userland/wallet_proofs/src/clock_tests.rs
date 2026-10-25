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
//! The wallet's clock in unix seconds: the wall clock's milliseconds over a
//! thousand, an unset clock as 0, and the budget and channel ends from it
//! in the unix range the meter and the chain count in, not YYYYMMDDhhmmss.

use nonos_payment_proto::Account;

use crate::clock_unix::unix_s;
use crate::meter_budget::budget_request;
use crate::meter_rows::budget_rows;
use crate::order::voucher_form::VoucherForm;

pub(crate) const NOW: u64 = 1_760_000_000;
/// The wall clock as nonos_tls::rtc_now gives it, for certificates only.
pub(crate) const RTC: u64 = 20_261_017_120_000;
/// 2100-01-01 in unix seconds: every end set from the clock is under it.
pub(crate) const UNIX_2100: u64 = 4_102_444_800;
const DAY: u64 = 86_400;

#[test]
fn milliseconds_read_as_unix_seconds_and_an_unset_clock_as_zero() {
    assert_eq!(unix_s(1_760_000_000_999), NOW);
    assert_eq!(unix_s(1_760_000_001_000), NOW + 1);
    assert_eq!(unix_s(-61), 0, "MkTimeMillis before the clock is set");
    assert_eq!(unix_s(0), 0);
    assert_eq!(unix_s(i64::MAX), i64::MAX as u64 / 1000);
    let form = VoucherForm { deposit: "1".into(), days: "1".into(), ..Default::default() };
    assert!(matches!(form.read(unix_s(-61)), Err(s) if s.contains("clock")));
    assert!(matches!(budget_request("1", "1", unix_s(-61)), Err(s) if s.contains("clock")));
}

#[test]
fn a_channel_and_a_budget_end_in_unix_seconds() {
    let form = VoucherForm { deposit: "25".into(), days: "30".into(), ..Default::default() };
    let expiry = form.read(NOW).map(|r| r.1);
    assert_eq!(expiry, Ok(NOW + 2_592_000));
    assert!(expiry.is_ok_and(|e| e < UNIX_2100));
    assert!(form.read(RTC).is_ok_and(|r| r.1 > UNIX_2100), "an rtc now is far past 2100");
    let b = budget_request("100", "7", NOW).map(|b| b.expiry);
    assert_eq!(b, Ok(NOW + 7 * DAY));
    assert!(b.is_ok_and(|e| e < UNIX_2100));
}

#[test]
fn the_meter_budget_end_reads_against_a_unix_now() {
    let a = Account {
        flags: 0,
        address: [0; 20],
        cap: 9,
        spent: 0,
        expiry: NOW + 3 * DAY,
        left: 9,
        total: 0,
        rows: vec![],
    };
    let nox = |w: u128| format!("{w}");
    assert_eq!(budget_rows(&a, NOW, nox)[3].1, "in 3 days");
    assert!(budget_rows(&a, RTC, nox)[3].1.starts_with("ended"), "an rtc now ends it");
}
