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

//! The presses beside those that move value or sign: each reads, reviews,
//! chooses, copies or leaves, and acts at once, unarmed.

use crate::wallet::arm::moves::{moves, Shown};
use crate::wallet::screen::hits::Press;

const ENTER: Option<Press> = None;

fn at(p: Press) -> Option<Press> {
    Some(p)
}

#[test]
fn presses_that_read_review_choose_or_leave_act_at_once() {
    use Press::{Back, Dismiss, Field, Footer, Row};
    let free = [
        (Shown::Consent, at(Footer(0)), "Close"),
        (Shown::Consent, at(Footer(2)), "Open another: the form only"),
        (Shown::Consent, at(Field(0)), "a field of the form"),
        (Shown::Escrow, at(Footer(0)), "Close"),
        (Shown::PayReview, at(Footer(1)), "back to edit"),
        (Shown::PayReview, ENTER, "Enter on a review"),
        (Shown::StakeReview, at(Back), "back"),
        (Shown::ShieldReview, at(Footer(1)), "back to edit"),
        (Shown::ShieldReview, at(Dismiss), "a reason put away"),
        (Shown::TakeBack(2), at(Footer(0)), "Stop, or a settle's review"),
        (Shown::Landed(12), at(Row(13)), "another row: its review"),
        (Shown::Meter, at(Footer(0)), "Refresh"),
        (Shown::Meter, at(Row(0)), "pick a channel"),
        (Shown::Publish, at(Row(0)), "the confirm box"),
        (Shown::Publish, at(Footer(0)), "Open the document"),
        (Shown::Export, ENTER, "a key on the export screen"),
    ];
    for (shown, press, what) in free {
        assert!(!moves(shown, press), "{what} on {shown:?} moves nothing");
    }
}
