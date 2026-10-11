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
    assert_eq!(c.charged(u128::MAX, ROOT, NOW), Err(MeterError::OverDeposit));
    let near = nonos_nox_meter::channel::Channel { cumulative: 1, deposit: u128::MAX, ..c };
    assert_eq!(near.charged(u128::MAX, ROOT, NOW), Err(MeterError::Overflow));
    let worn = nonos_nox_meter::channel::Channel { seq: u64::MAX, ..c };
    assert_eq!(worn.charged(1, ROOT, NOW), Err(MeterError::Overflow));
}

#[test]
fn what_the_chain_settled_is_a_floor_under_cumulative() {
    let c = channel().charged(100, ROOT, NOW).unwrap();
    let (synced, rose) = c.synced(1200, 400);
    assert_eq!((synced.cumulative, synced.settled, synced.deposit, rose), (400, 400, 1200, 300));
    // The next voucher starts above what was already paid.
    assert_eq!(synced.charged(10, ROOT, NOW).unwrap().cumulative, 410);
    // A lower report never lowers anything.
    let (again, rose) = synced.synced(1200, 50);
    assert_eq!((again.cumulative, again.settled, rose), (400, 400, 0));
}
