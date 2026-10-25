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

/* The publisher's presses under the guard (as press_grey.rs): a
 * transaction's Sign and send is held to the arming and drawn greyed until
 * armed; Cancel and Enter are not; the Bond's and a release's rows only ask
 * for a quote and act at once, while the binding and attests stay signed
 * under the guard. */

use super::super::order_tests::press_grey::{LOOKS, T};
use crate::order::terms::arming::ARM_MS;
use crate::wallet::arm::look::{painted, pressed, seen, Arm};
use crate::wallet::arm::moves::{moves, Shown};
use crate::wallet::arm::rows::{ROW_ATTEST, ROW_BINDING, ROW_INTENT, ROW_RELEASE};
use crate::wallet::arm::shows::greys;
use crate::wallet::screen::hits::Press::{Back, Field, Footer, Row};

#[test]
fn sign_and_send_is_guarded_and_nothing_else_on_the_review() {
    assert!(moves(Shown::PublisherReview, Some(Footer(0))), "Sign and send");
    let free = [Some(Footer(1)), Some(Back), Some(Field(1)), Some(Row(ROW_INTENT)), None];
    for (i, press) in free.into_iter().enumerate() {
        assert!(!moves(Shown::PublisherReview, press), "case {i} acts at once");
    }
}

#[test]
fn the_rows_that_only_quote_are_not_guarded_and_the_signing_rows_are() {
    for r in ROW_INTENT..=ROW_RELEASE + 3 * 16 {
        assert!(!moves(Shown::Publish, Some(Row(r))), "row {r} only asks for a quote");
    }
    assert!(moves(Shown::Publish, Some(Row(ROW_BINDING))));
    for r in ROW_ATTEST..ROW_ATTEST + 16 {
        assert!(moves(Shown::Publish, Some(Row(r))), "attest row {r}");
    }
    for f in 0..4 {
        assert!(!moves(Shown::Publish, Some(Field(f))), "typing into field {f}");
    }
}

#[test]
fn sign_and_send_is_greyed_exactly_while_held_back() {
    let (shown, press) = (Some(Shown::PublisherReview), Some(Footer(0)));
    let undrawn = seen(Arm::default(), shown, true, T - 1);
    assert!(greys(&undrawn, shown, press, T + 5 * ARM_MS), "not drawn: greyed");
    let mut a = painted(undrawn, shown);
    assert!(greys(&a, shown, press, T), "drawn, before any look: greyed");
    for (now, focused, acts) in LOOKS {
        let gate = pressed(a, shown, focused, now);
        a = seen(a, shown, focused, now);
        assert_eq!(gate, acts, "gate at {now}, focused {focused}");
        assert_eq!(greys(&a, shown, press, now), !gate, "paint at {now}");
    }
    assert!(!greys(&a, shown, Some(Footer(1)), T), "Cancel never greys");
}
