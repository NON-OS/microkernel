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
//! A refresh begins only once the user has paused, so no network step starts
//! under the hand, and it still keeps its interval once they have. This
//! checks the wallet's own gate (app_refresh_due.rs) with its own constants.

use crate::refresh_due::{due, IDLE_MS, REFRESH_MS};

const NOW: i64 = 1_000_000;

// The pause is shorter than the interval: a user who stops gets a refresh
// within one interval.
const _: () = assert!(IDLE_MS > 0 && IDLE_MS < REFRESH_MS);

#[test]
fn no_refresh_begins_while_the_user_is_active() {
    // A click or keypress within IDLE_MS defers it, owed or not.
    for since in [0, 1, IDLE_MS / 2, IDLE_MS - 1] {
        assert!(!due(true, true, NOW, NOW - since, 0), "active {since} ms ago");
    }
}

#[test]
fn an_owed_refresh_begins_once_the_user_pauses() {
    // The first refresh, or one the status step asks for, does not wait the
    // interval.
    assert!(due(true, true, NOW, NOW - IDLE_MS, NOW));
}

#[test]
fn refreshes_keep_their_interval() {
    let paused = NOW - IDLE_MS;
    assert!(!due(true, false, NOW, paused, NOW - (REFRESH_MS - 1)), "not before it");
    assert!(due(true, false, NOW, paused, NOW - REFRESH_MS), "once it has passed");
}

#[test]
fn no_refresh_begins_where_nothing_live_is_shown() {
    for last_probe in [0, NOW - REFRESH_MS, NOW] {
        assert!(!due(false, true, NOW, 0, last_probe));
    }
}
