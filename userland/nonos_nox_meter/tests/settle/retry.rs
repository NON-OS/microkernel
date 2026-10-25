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

//! The daily retry against a `last_retry` behind, at or ahead of `now`.

use nonos_nox_meter::settle::{settle_plan, RETRY_EVERY};
use nonos_nox_meter::MeterError;

use super::fixture::{sign, state, CHAIN, METER, NOW, SETTLEMENT};

/// Whether the same vouchers at a passed nonce are signed again at `now`;
/// when they are, the `last_retry` the state keeps.
fn retry(last_retry: u64, now: u64) -> Option<u64> {
    let mut s = state();
    (s.last_settle, s.last_retry) = (Some((CHAIN, 5)), last_retry);
    s.channels.iter_mut().for_each(|c| c.carried = c.cumulative);
    match settle_plan(&s, &METER, &sign(6, &[1, 2]), &SETTLEMENT, now) {
        Ok((next, _)) => Some(next.last_retry),
        Err(e) => {
            assert_eq!(e, MeterError::NothingOwed);
            None
        }
    }
}

#[test]
fn a_past_stamp_is_due_a_day_on_and_not_a_second_before() {
    assert_eq!(retry(NOW, NOW + RETRY_EVERY - 1), None);
    assert_eq!(retry(NOW, NOW + RETRY_EVERY), Some(NOW + RETRY_EVERY));
    assert_eq!(retry(NOW, NOW), None);
}

#[test]
fn a_stamp_more_than_a_day_ahead_is_due_and_restamped_now() {
    assert_eq!(retry(NOW + RETRY_EVERY + 1, NOW), Some(NOW));
    assert_eq!(retry(u64::MAX, NOW), Some(NOW));
}

#[test]
fn a_stamp_up_to_a_day_ahead_is_not_due() {
    assert_eq!(retry(NOW + RETRY_EVERY, NOW), None);
    assert_eq!(retry(NOW + 1, NOW), None);
}

#[test]
fn the_ends_of_the_clock_do_not_wrap() {
    assert_eq!(retry(0, 0), None);
    assert_eq!(retry(0, u64::MAX), Some(u64::MAX));
    assert_eq!(retry(u64::MAX, 0), Some(0));
    assert_eq!(retry(u64::MAX - 1, u64::MAX - 2), None, "a wrapped day would be due");
    assert_eq!(retry(RETRY_EVERY, 0), None, "a wrapped ahead would be due");
    // Today's saturating rule: a day past u64::MAX - RETRY_EVERY is the end.
    assert_eq!(retry(u64::MAX, u64::MAX), Some(u64::MAX));
}
