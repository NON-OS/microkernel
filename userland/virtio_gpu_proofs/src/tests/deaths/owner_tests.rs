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
//! The owner as the watch sees it: due at OWNER_SILENCE_MS of silence, or
//! at once when its process is gone; never with no owner; once a blank
//! landed whole, not again until it is heard and silent again; LOST its
//! own, once; and a new owner starts with nothing of the last one's.

use crate::state::owner::{Owner, OWNER_SILENCE_MS};

#[test]
fn an_owner_is_due_when_silent_long_enough_or_dead() {
    let o = Owner::new();
    assert!(!o.due(u64::MAX, false), "no owner, nothing to blank");
    o.adopt(7, 1_000);
    assert!(!o.due(1_000 + OWNER_SILENCE_MS - 1, true));
    assert!(o.due(1_000 + OWNER_SILENCE_MS, true), "silent");
    assert!(o.due(1_001, false), "dead");
    o.heard(8, 9_000);
    assert!(o.due(9_000, true), "another pid is not the owner heard");
    o.quiet.set(true);
    assert!(!o.due(u64::MAX, false), "blanked whole: not again");
    o.heard(7, 9_000);
    assert!(!o.quiet.get() && !o.due(9_000, true) && o.due(9_000 + OWNER_SILENCE_MS, true));
}

#[test]
fn lost_is_the_owner_s_once_and_a_new_owner_starts_clean() {
    let o = Owner::new();
    o.adopt(7, 0);
    o.lost.set(true);
    assert!(!o.take_lost(8) && o.take_lost(7) && !o.take_lost(7));
    o.lost.set(true);
    o.quiet.set(true);
    o.adopt(9, 50);
    assert_eq!(
        (o.pid.get(), o.heard_ms.get(), o.quiet.get(), o.take_lost(9)),
        (9, 50, false, false)
    );
}
