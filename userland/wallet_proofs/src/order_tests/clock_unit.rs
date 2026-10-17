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
//! The purchase's clock is unix seconds (B8): a channel a day ahead is open,
//! OPEN START takes an expiry from the form, and the refund, its due mark
//! and the gas wait count from the order's deadline and the paid time.

use nonos_payment_proto::OrderStatus;

use super::fixture::{buy, made, offer as order_offer, REFUND_PAIR, REFUND_TO};
use super::held::{page, row};
use super::voucher::voucher;
use crate::order::escrow_offer::{offer, Offer};
use crate::order::held::{after_page, open_for, Held};
use crate::order::open_gate::open_start;
use crate::order::refund::deadline;
use crate::order::send_resume::{gas_offer, GAS_WAIT_S};
use crate::order::voucher_form::VoucherForm;
use crate::shield::due_why::due_why;
use crate::tests_market::clock_tests::{NOW, RTC, UNIX_2100};

#[test]
fn a_channel_a_day_ahead_is_open_at_a_unix_now() {
    let v = voucher();
    let r = row(v.listing_id, v.chain_id, NOW + 86_400, false);
    assert!(open_for(&r, &v, NOW));
    assert_eq!(after_page(Some(&page(1, vec![r])), 0, &v, NOW), Held::Open(r));
    assert!(!open_for(&r, &v, RTC), "an rtc now finds no channel open");
}

#[test]
fn open_start_carries_the_form_expiry_in_unix_seconds() {
    let v = voucher();
    let form = VoucherForm { deposit: "25".into(), days: "30".into(), ..Default::default() };
    let (amount, expiry, _) = form.read(NOW).unwrap();
    let s = open_start(&v, Ok(REFUND_TO), amount, expiry, || NOW).unwrap();
    assert_eq!(s.expiry, NOW + 30 * 86_400);
    assert!(s.expiry < UNIX_2100);
    let late = open_start(&v, Ok(REFUND_TO), amount, expiry, || RTC);
    assert!(late.is_err(), "a unix expiry is before an rtc now");
}

#[test]
fn a_funded_order_owes_its_refund_from_its_unix_deadline() {
    let m = made(&buy().order_new(&order_offer(), REFUND_PAIR, NOW));
    let due = deadline(&m).unwrap();
    assert!(due > NOW && due < UNIX_2100, "{due}");
    let s = OrderStatus { state: 1, funded: 9, pending: 0, extra: 0, plain: false };
    assert_eq!(offer(None, Some(&s), None, due, NOW), None, "in time");
    assert_eq!(offer(None, Some(&s), None, due, due), Some(Offer::Refund));
    assert_eq!(due_why(None, Some(&s), None, due, NOW), None, "not due in time");
    assert!(due_why(None, Some(&s), None, due, due).is_some());
    assert_eq!(offer(None, Some(&s), None, due, RTC), Some(Offer::Refund), "rtc: at once");
}

#[test]
fn the_gas_wait_is_ten_minutes_of_unix_seconds() {
    let pay = Some(Offer::PayGas);
    assert_eq!(gas_offer(pay, Some(NOW), NOW + GAS_WAIT_S - 1), Some(Offer::GasWaiting));
    assert_eq!(gas_offer(pay, Some(NOW), NOW + GAS_WAIT_S), pay);
    assert_eq!(GAS_WAIT_S, 600);
}
