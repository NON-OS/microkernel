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

//! How long the first paint waits for a page's stylesheets.
//!
//! A stylesheet is render-blocking: the page is laid out once its sheets are
//! in, so the reader never sees it unstyled and then restyled. That costs
//! nothing when sheets come in a few hundred milliseconds. Through Anyone or
//! Nym each sheet is a stream of its own, and a site such as github.com pulls
//! in a dozen of them, four at a time: on a slow machine the page stayed
//! blank for minutes, read as a page that does not load at all. The wait is
//! now bounded. Past it the page is laid out with the sheets that have come,
//! its scripts run, and each sheet still to come restyles it as it lands.

/// The longest the first paint waits for stylesheets.
pub const STYLE_HOLD_MS: i64 = 5_000;

/// The shortest gap between two restyles for sheets that land after the
/// page is shown. A restyle of a large page is the cascade and layout over
/// again, most of a second on a slow machine; a dozen sheets landing close
/// together are taken up in one.
pub const RESTYLE_GAP_MS: i64 = 1_000;

/// Where a page stands with its render-blocking stylesheets.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StyleHold {
    /// When the wait ends, on the uptime clock, while it runs.
    pub due: Option<i64>,
    /// The wait ran out: the page is shown and each sheet restyles it.
    pub over: bool,
    /// A sheet landed since the last restyle.
    pub stale: bool,
    /// When the page was last restyled for a late sheet.
    pub restyled: i64,
}

impl StyleHold {
    /// A wait that starts at `now`.
    pub fn from(now: i64) -> StyleHold {
        StyleHold { due: Some(now.saturating_add(STYLE_HOLD_MS)), ..StyleHold::default() }
    }

    /// End the wait if it has run out at `now`. True the one time it does.
    pub fn lapse(&mut self, now: i64) -> bool {
        match self.due {
            Some(due) if now >= due => {
                self.due = None;
                self.over = true;
                self.restyled = now;
                true
            }
            _ => false,
        }
    }

    /// Whether a late sheet's restyle is due at `now`: one has landed, and
    /// the last restyle was at least `RESTYLE_GAP_MS` ago. Clears the mark
    /// when it says yes.
    pub fn restyle(&mut self, now: i64) -> bool {
        if !self.stale || now.wrapping_sub(self.restyled) < RESTYLE_GAP_MS {
            return false;
        }
        self.stale = false;
        self.restyled = now;
        true
    }
}
