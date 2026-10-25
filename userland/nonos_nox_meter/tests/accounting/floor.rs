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

//! What settle's report does to a channel (plan/21, v1.3 E13).

use super::fixture::{channel, funded_state, LISTING, NOW, PAYEE, ROOT};
use nonos_nox_meter::channel::{Channel, CHARGE_MARGIN};
use nonos_nox_meter::state::ChainView;
use nonos_nox_meter::MeterError;

#[test]
fn a_rolled_back_file_freezes_its_channel_and_another_takes_the_charge() {
    // The chain settled 300 on a channel this file says nothing was signed on.
    let s = funded_state().synced(&channel().id, 1000, 300);
    assert!(s.channels[0].frozen);
    assert_eq!((s.channels[0].cumulative, s.budget.spent), (0, 0));
    assert_eq!(s.charged(&PAYEE, &LISTING, 1, 1, ROOT, NOW).err(), Some(MeterError::Frozen));
    let other = Channel { id: [8; 32], ..channel() };
    let (_, c) =
        s.with_channel(other, NOW).unwrap().charged(&PAYEE, &LISTING, 1, 1, ROOT, NOW).unwrap();
    assert_eq!(c.id, other.id);
}

#[test]
fn the_chain_closes_a_channel_and_a_charge_prefers_one_with_room() {
    let v = ChainView { deposit: 1000, settled: 200, expiry: channel().expiry, closed: true };
    let s = funded_state().from_chain(&channel().id, &v);
    // Settled 200 on a channel nothing was signed on: frozen, and closed.
    assert_eq!((s.channels[0].deposit, s.channels[0].frozen), (200, true));
    assert_eq!(s.charged(&PAYEE, &LISTING, 1, 1, ROOT, NOW).err(), Some(MeterError::Frozen));
    let fresh = Channel { id: [7; 32], expiry: NOW + CHARGE_MARGIN + 60, ..channel() };
    let (_, c) =
        s.with_channel(fresh, NOW).unwrap().charged(&PAYEE, &LISTING, 1, 1, ROOT, NOW).unwrap();
    assert_eq!(c.id, fresh.id);
}
