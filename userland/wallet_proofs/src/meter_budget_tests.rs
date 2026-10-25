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

//! The meter's budget form (B8): whole NOX and days as typed, into the
//! SET_BUDGET the meter reads, and nothing it would misread.

use nonos_payment_proto::SetBudget;

use crate::meter_budget::{budget_request, push_digit, NO_CAP, NO_CLOCK, NO_DAYS};

const NOW: u64 = 1_800_000_000;
const E18: u128 = 1_000_000_000_000_000_000;

#[test]
fn a_cap_and_days_become_wei_and_an_end() {
    let b = budget_request("100", "7", NOW).unwrap();
    assert_eq!(b, SetBudget { cap: 100 * E18, expiry: NOW + 7 * 86_400, listings: vec![] });
    let wire = b.encode().unwrap();
    assert_eq!(SetBudget::parse(&wire), Some(b), "what the meter reads back");
    let most = budget_request("1000000000000", "365", NOW).unwrap();
    assert_eq!(most.cap, 1_000_000_000_000 * E18);
}

#[test]
fn nothing_outside_the_bounds_or_without_a_clock_is_sent() {
    for (cap, days, why) in [
        ("", "7", NO_CAP),
        ("0", "7", NO_CAP),
        ("1000000000001", "7", NO_CAP),
        ("1.5", "7", NO_CAP),
        ("100", "0", NO_DAYS),
        ("100", "366", NO_DAYS),
        ("100", "", NO_DAYS),
    ] {
        assert_eq!(budget_request(cap, days, NOW), Err(why), "{cap} {days}");
    }
    assert_eq!(budget_request("100", "7", 0), Err(NO_CLOCK));
}

#[test]
fn a_field_takes_digits_up_to_its_length_and_no_leading_zero() {
    let mut t = String::new();
    assert!(!push_digit(&mut t, '0' as u32, 3), "no leading zero");
    assert!(push_digit(&mut t, '1' as u32, 3) && push_digit(&mut t, '0' as u32, 3));
    assert!(!push_digit(&mut t, 'x' as u32, 3) && !push_digit(&mut t, '.' as u32, 3));
    assert!(push_digit(&mut t, '5' as u32, 3) && !push_digit(&mut t, '5' as u32, 3));
    assert_eq!(t, "105");
}
