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

//! The model the F-108 run is held to, apart from the watch: when each
//! watch's condition was first reported, and what a report or the clock
//! alone should do.

use super::super::critical_watch::{Action, GRACE_MS};

/// The model of one watch: when its condition was first reported.
pub fn model(armed: &mut Option<u64>, bad: bool, now: u64) -> Action {
    match (bad, *armed) {
        (false, _) => (*armed = None, Action::Nothing).1,
        (true, None) => (*armed = Some(now.max(1)), Action::AskShutdown).1,
        (true, Some(t)) if now - t >= GRACE_MS => Action::PowerOff,
        (true, Some(_)) => Action::Nothing,
    }
}

pub fn late(armed: Option<u64>, now: u64) -> Action {
    match armed {
        Some(t) if now - t >= GRACE_MS => Action::PowerOff,
        _ => Action::Nothing,
    }
}

pub fn roll(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n
}
