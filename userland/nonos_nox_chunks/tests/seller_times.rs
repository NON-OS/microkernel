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
#[path = "common/order_fixture.rs"]
mod order_fixture;

use nonos_nox_chunks::{may_send, TermsWrong};
use nonos_nox_config::{DELIVER_MARGIN, MAX_DELIVERY};
use order_fixture::{check, terms, NOW};

#[test]
fn the_deadline_and_window_are_inside_the_rules() {
    use TermsWrong::*;
    assert_eq!(check(|t| t.dispute_window += 1), Err(BadTimes));
    assert_eq!(check(|t| t.delivery_deadline = NOW + DELIVER_MARGIN), Ok(()));
    assert_eq!(check(|t| t.delivery_deadline = NOW + DELIVER_MARGIN - 1), Err(BadTimes));
    assert_eq!(check(|t| t.delivery_deadline = NOW + MAX_DELIVERY), Ok(()));
    assert_eq!(check(|t| t.delivery_deadline = NOW + MAX_DELIVERY + 1), Err(BadTimes));
}

#[test]
fn nothing_is_sent_inside_the_margin() {
    let t = terms();
    assert!(may_send(&t, t.delivery_deadline - DELIVER_MARGIN));
    assert!(!may_send(&t, t.delivery_deadline - DELIVER_MARGIN + 1));
    assert!(!may_send(&t, u64::MAX));
}
