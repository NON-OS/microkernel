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

//! Off the purchase a guarded press is drawn greyed until it arms, by the
//! fn the gate asks (arm/shows.rs): greyed exactly while held back.

use crate::order::terms::arming::ARM_MS;
use crate::wallet::arm::look::{painted, pressed, seen, Arm};
use crate::wallet::arm::moves::Shown;
use crate::wallet::arm::rows::{ROW_ATTEST, ROW_BINDING, ROW_CLAIM, ROW_CLOSE, ROW_TOPUP};
use crate::wallet::arm::shows::greys;
use crate::wallet::screen::hits::Press::{self, Footer, Row};

pub const T: u64 = 90_000;

/// Each look: when, whether focused, and whether the gate lets it act.
pub const LOOKS: [(u64, bool, bool); 7] = [
    (T, true, false),
    (T + ARM_MS - 1, true, false),
    (T + ARM_MS, true, true),
    (T + ARM_MS + 5, false, false),
    (T + ARM_MS + 10, true, false),
    (T + 2 * ARM_MS + 9, true, false),
    (T + 2 * ARM_MS + 10, true, true),
];

pub const GUARDED: [(Shown, Option<Press>); 16] = [
    (Shown::Budget, Some(Footer(1))),
    (Shown::Escrow, Some(Footer(1))),
    (Shown::PayReview, Some(Footer(0))),
    (Shown::StakeReview, Some(Footer(0))),
    (Shown::ShieldReview, Some(Footer(0))),
    (Shown::TakeBack(2), Some(Footer(2))),
    (Shown::Landed(12), Some(Row(12))),
    (Shown::Meter, Some(Footer(1))),
    (Shown::Meter, None),
    (Shown::Meter, Some(Row(ROW_TOPUP))),
    (Shown::Meter, Some(Row(ROW_CLOSE))),
    (Shown::Meter, Some(Row(ROW_CLAIM))),
    (Shown::Publish, Some(Row(ROW_BINDING))),
    (Shown::Publish, Some(Row(ROW_ATTEST))),
    (Shown::Publish, Some(Row(ROW_ATTEST + 3))),
    (Shown::Export, Some(Footer(0))),
];

#[test]
fn a_guarded_press_is_drawn_greyed_exactly_while_the_gate_holds_it_back() {
    for (i, (s, press)) in GUARDED.into_iter().enumerate() {
        let shown = Some(s);
        let undrawn = seen(Arm::default(), shown, true, T - 1);
        assert!(greys(&undrawn, shown, press, T + 5 * ARM_MS), "{s:?} not drawn: greyed");
        let mut a = painted(undrawn, shown);
        assert!(greys(&a, shown, press, T), "{s:?} drawn, before any look: greyed");
        for (now, focused, acts) in LOOKS {
            let gate = pressed(a, shown, focused, now);
            a = seen(a, shown, focused, now);
            let grey = greys(&a, shown, press, now);
            assert_eq!(gate, acts, "{s:?} case {i} gate at {now}, focused {focused}");
            assert_eq!(grey, !gate, "{s:?} case {i} paint at {now}, focused {focused}");
        }
    }
}
