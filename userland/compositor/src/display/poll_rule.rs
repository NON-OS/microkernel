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

//! What a poll of a wire driver does (section 12): every 500 ms the
//! driver's service is looked up again and, when it is the same process,
//! its STATE read. A new pid is a restarted driver whose primaries are all
//! gone; a new serial is a primary the compositor has not attached; LOST
//! is an output the driver programmed again, whose picture must be redrawn.

use nonos_gfx_wire::body::state::LOST;
use nonos_gfx_wire::State;

use super::link::WireLink;

pub const POLL_MS: i64 = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Keep,
    /// Damage the whole canvas.
    Repaint,
    /// Ask SURFACE again of the same driver and attach what it answers.
    Resurface,
    /// Drive the driver now announced from the start: CAPS, then SURFACE.
    Reprime,
}

/// `found`: the pid the service is announced under now, None while it is
/// not. `state`: STATE's answer, read only when the pid is the same.
pub fn step(link: &WireLink, found: Option<u32>, state: Option<State>) -> Step {
    match found {
        // Gone: FLUSH fails and the damage waits for the driver to return.
        None => return Step::Keep,
        Some(pid) if pid != link.pid => return Step::Reprime,
        Some(_) => {}
    }
    if link.stale {
        return Step::Resurface;
    }
    match state {
        Some(s) if s.serial != link.serial => Step::Resurface,
        Some(s) if s.flags & LOST != 0 => Step::Repaint,
        _ => Step::Keep,
    }
}
