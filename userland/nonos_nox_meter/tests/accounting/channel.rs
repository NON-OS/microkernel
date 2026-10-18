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

use super::fixture::{channel, NOW, ROOT};
use nonos_nox_meter::channel::CHARGE_MARGIN;
use nonos_nox_meter::MeterError;

#[test]
fn a_charge_raises_cumulative_and_seq_and_nothing_else() {
    let c = channel().charged(250, ROOT, NOW).unwrap();
    assert_eq!((c.cumulative, c.seq), (250, 1));
    let c = c.charged(750, ROOT, NOW).unwrap();
    assert_eq!((c.cumulative, c.seq), (1000, 2));
    assert_eq!(c.charged(1, ROOT, NOW), Err(MeterError::OverDeposit));
}

#[test]
fn nothing_is_charged_at_or_past_expiry_or_past_the_numbers() {
    let c = channel();
    assert_eq!(c.charged(1, ROOT, c.expiry), Err(MeterError::Expired));
    let last_day = c.expiry - CHARGE_MARGIN;
    assert_eq!(c.charged(1, ROOT, last_day), Err(MeterError::Expired), "within a day");
    assert!(c.charged(1, ROOT, last_day - 1).is_ok());
    assert_eq!(c.charged(u128::MAX, ROOT, NOW), Err(MeterError::OverDeposit));
    let near = nonos_nox_meter::channel::Channel { cumulative: 1, deposit: u128::MAX, ..c };
    assert_eq!(near.charged(u128::MAX, ROOT, NOW), Err(MeterError::Overflow));
    let worn = nonos_nox_meter::channel::Channel { seq: u64::MAX, ..c };
    assert_eq!(worn.charged(1, ROOT, NOW), Err(MeterError::Overflow));
}

#[test]
fn settled_raises_the_floor_only_up_to_what_this_meter_signed() {
    let c = channel().charged(500, ROOT, NOW).unwrap();
    let synced = c.synced(1200, 400);
    assert_eq!(
        (synced.cumulative, synced.settled, synced.deposit, synced.frozen),
        (500, 400, 1200, false)
    );
    // The next voucher starts above what was already signed.
    assert_eq!(synced.charged(10, ROOT, NOW).unwrap().cumulative, 510);
    // A lower report never lowers anything.
    assert_eq!(synced.synced(1200, 50).settled, 400);
    // More than this meter ever signed: frozen, numbers untouched.
    let lied = synced.synced(1200, 501);
    assert!(lied.frozen);
    assert_eq!((lied.cumulative, lied.settled), (500, 400));
    assert_eq!(lied.charged(1, ROOT, NOW), Err(MeterError::Frozen));
}
