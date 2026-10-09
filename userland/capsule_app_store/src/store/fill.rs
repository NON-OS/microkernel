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

//! Asking the market about listings, one step per tick.
//!
//! A selection used to cost three market calls on the spot, and so did every
//! keystroke in the search field, since typing moves the cursor: with a slow
//! market, each letter froze the window for up to three timeouts. Now a
//! key or a click only moves the cursor and the next tick asks, once per
//! listing per catalogue. The selected listing goes first; after it, each
//! listing's description is asked in turn so the search can match it.

use super::fill_order::next;
use super::market::{self, Failure};
use super::state::State;

/// How soon the next step runs while there is one: quick enough that the
/// pane fills as the cursor lands, without a busy loop.
pub const STEP_MS: i64 = 30;

/// What a step did.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Nothing left to ask.
    Done,
    /// Asked about a listing; the screen shows what changed.
    Shown,
    /// Asked about a listing nothing on screen depends on yet.
    Quiet,
}

impl State {
    /// One step of asking. A new description can change what the search
    /// keeps, so it is shown whenever a query is typed.
    pub fn fill(&mut self) -> Step {
        if !self.filling() {
            return Step::Done;
        }
        let Some((at, selected)) = next(&self.listings, self.current_index()) else {
            return Step::Done;
        };
        let port = self.port;
        let Some(l) = self.listings.get_mut(at) else { return Step::Done };
        let mut silent = false;
        if !l.known.described {
            l.known.detail = kept(market::get_app(port, market::next_id(), &l.id), &mut silent);
            l.known.described = true;
        }
        if selected {
            let release = market::get_release(port, market::next_id(), &l.id);
            l.known.release = kept(release, &mut silent);
            let readiness = market::install_ready(port, market::next_id(), &l.id);
            l.known.readiness = kept(readiness, &mut silent);
            l.known.judged = true;
        }
        /*
         * A market that has stopped answering costs a timeout per call, and
         * there is one call per listing still to ask: stop asking until `r`
         * finds it again, so the window is not frozen a timeout at a time.
         */
        if silent {
            self.port = 0;
        }
        match selected || silent || !self.search.text().is_empty() {
            true => Step::Shown,
            false => Step::Quiet,
        }
    }

    /// Whether the market still has something to be asked. False once it
    /// has stopped answering (`port` zero), since every answer would be the
    /// same silence.
    pub fn filling(&self) -> bool {
        self.port != 0 && next(&self.listings, self.current_index()).is_some()
    }
}

/// The answer, noting when there was none at all.
fn kept<T>(got: Result<T, Failure>, silent: &mut bool) -> Option<T> {
    *silent |= matches!(got, Err(Failure::NoReply));
    got.ok()
}
