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

//! bootfb's hold, one step each. HELD is stored under the claims' lock as
//! its claim is made ready, and the wait runs once that lock is let go, so
//! a release of the claim, taking the lock the wait no longer holds, can
//! clear HELD before the wait reads DRAWS at 0.

use super::actors::Hold;
use super::events::Event;
use super::world::Flags;

/// One step of bootfb's hold; `early` is the release clearing HELD while
/// the wait still runs.
pub fn hold(f: &mut Flags, h: &mut Hold, early: bool) -> Option<Event> {
    let (next, e) = match *h {
        Hold::Start => {
            f.held = true;
            (Hold::Set, Event::Held(true))
        }
        Hold::Set if early => {
            f.held = false;
            (Hold::Waiting, Event::Held(false))
        }
        Hold::Set if f.draws == 0 => (Hold::Quiet, Event::Quiet),
        Hold::Waiting if f.draws == 0 => (Hold::Cleared, Event::Quiet),
        Hold::Quiet => {
            f.held = false;
            (Hold::Cleared, Event::Held(false))
        }
        _ => return None,
    };
    *h = next;
    Some(e)
}
