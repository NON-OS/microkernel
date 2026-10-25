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

//! A pending scan or join advanced by one tick. The next tick paints
//! "Scanning..." or "Joining <name>..." on the window, and the tick after
//! hands the call to the worker; the request is then out, and stays on the
//! panel, until a later tick finds the answer. A request no tick has picked
//! up in `STALE_MS` (the window was hidden, say) is dropped rather than run
//! long after it was asked for. A request already out is not dropped: the
//! driver's own budget (15 s for a scan, 30 s for a join) ends it.

use super::pending::{Pending, Work};

/// What this tick should do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    Nothing,
    /// Repaint, so the waiting state is on screen before the call.
    Show,
    /// Run the call now.
    Run(Work),
    /// Too old to run; drop it and say so.
    Expired(Work),
}

/// A request older than this when its turn comes is dropped.
pub const STALE_MS: i64 = 10_000;

impl Pending {
    /// Advance one tick. A request is shown on the first tick after it was
    /// asked and run on the next, never both in one. One that is out waits
    /// for its answer (`Nothing`); the caller looks for that itself.
    pub fn tick(&mut self, now_ms: i64) -> Next {
        let Pending::Asked { work, since_ms, shown } = *self else {
            return Next::Nothing;
        };
        if now_ms.saturating_sub(since_ms) >= STALE_MS {
            *self = Pending::Idle;
            return Next::Expired(work);
        }
        if !shown {
            *self = Pending::Asked { work, since_ms, shown: true };
            return Next::Show;
        }
        *self = Pending::Idle;
        Next::Run(work)
    }
}
