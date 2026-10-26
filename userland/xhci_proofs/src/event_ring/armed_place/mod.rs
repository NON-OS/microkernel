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

//! The polled bulk IN and an isoch batch on the event ring the interrupt
//! reports share: each keeps a place for its event, so another wait that
//! reads it first leaves it for them, and with no place free nothing is
//! queued. A full isoch batch fits beside the one the controller plays.

mod bulk_tests;
mod isoch_tests;

use super::fixture::{Fixture, INT_DCI};
use crate::rings::event::state::ARMED;
use crate::rings::event::IssuedTransfer;

/// Every place for an armed TRB's event taken, by interrupt endpoints of
/// other slots, two on each.
fn take_every_place(fx: &mut Fixture) {
    for n in 0..ARMED {
        let (slot, dci) = (100 + (n / 2) as u8, INT_DCI + 2 * (n % 2) as u8);
        assert!(fx.ring.expect(IssuedTransfer { phys: 0x10_0000, slot, dci }));
    }
    assert!(!fx.ring.can_expect());
}
