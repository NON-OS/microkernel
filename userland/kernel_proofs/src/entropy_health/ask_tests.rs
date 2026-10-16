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

//! The ask: one out at a time, given up when unanswered, and only its own
//! answer counted.

use super::ask_rule::{answers, due, Asked, GIVE_UP_MS};

const OUT: Asked = Asked { id: 4, generation: 2, at_ms: 1_000 };

#[test]
fn an_ask_goes_only_to_a_live_capsule_when_bytes_are_wanted() {
    assert!(due(None, 0, 2, true, true));
    assert!(!due(None, 0, 2, false, true), "dead");
    assert!(!due(None, 0, 2, true, false), "nothing wanted");
}

#[test]
fn one_ask_is_out_until_answered_or_given_up() {
    assert!(!due(Some(OUT), 1_000 + GIVE_UP_MS - 1, 2, true, true));
    assert!(due(Some(OUT), 1_000 + GIVE_UP_MS, 2, true, true), "given up");
    assert!(due(Some(OUT), 1_001, 3, true, true), "the capsule restarted");
}

#[test]
fn only_the_ask_out_is_answered() {
    assert!(answers(Some(OUT), 4, 2));
    assert!(!answers(Some(OUT), 5, 2), "another ask's id");
    assert!(!answers(Some(OUT), 4, 3), "a restarted capsule");
    assert!(!answers(None, 4, 2), "nothing out");
}
