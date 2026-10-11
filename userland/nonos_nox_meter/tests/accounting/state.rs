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

use super::fixture::{channel, funded_state, LISTING, NOW, PAYEE, ROOT};
use nonos_nox_meter::channel::Channel;
use nonos_nox_meter::MeterError;

#[test]
fn a_charge_needs_a_channel_a_deposit_and_a_budget() {
    let s = funded_state();
    assert_eq!(
        s.charged(&[0xcc; 20], &LISTING, 1, 1, ROOT, NOW).err(),
        Some(MeterError::NoChannel)
    );
    assert_eq!(
        s.charged(&PAYEE, &LISTING, 11_155_111, 1, ROOT, NOW).err(),
        Some(MeterError::NoChannel)
    );
    let (s, c) = s.charged(&PAYEE, &LISTING, 1, 400, ROOT, NOW).unwrap();
    assert_eq!((c.cumulative, s.budget.spent, s.channels[0].seq), (400, 400, 1));
    // The deposit would allow 600 more; the budget allows 100.
    assert_eq!(s.charged(&PAYEE, &LISTING, 1, 101, ROOT, NOW).err(), Some(MeterError::OverBudget));
}

#[test]
fn a_refused_charge_changes_nothing() {
    let s = funded_state();
    let before = s.clone();
    let _ = s.charged(&PAYEE, &LISTING, 1, 501, ROOT, NOW);
    assert_eq!(s, before);
}

#[test]
fn reopening_a_channel_never_rewinds_what_was_signed() {
    let (s, _) = funded_state().charged(&PAYEE, &LISTING, 1, 250, ROOT, NOW).unwrap();
    let s = s.with_channel(Channel { deposit: 2000, ..channel() }, NOW).unwrap();
    assert_eq!(
        (s.channels[0].cumulative, s.channels[0].seq, s.channels[0].deposit),
        (250, 1, 2000)
    );
}
