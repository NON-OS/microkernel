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

use super::fill_order::{next, recheck};
use super::fill_step::{kept, now_ms, Step};
use super::market;
use super::state::State;

impl State {
    /// One step of asking. A new description can change what the search
    /// keeps, so it is shown whenever a query is typed.
    pub fn fill(&mut self) -> Step {
        if !self.filling() {
            return Step::Done;
        }
        let now = now_ms();
        let Some((at, selected)) = next(&self.listings, self.current_index(), now) else {
            return Step::Done;
        };
        let port = self.port;
        let Some(l) = self.listings.get_mut(at) else { return Step::Done };
        let mut silent = false;
        if !l.known.described {
            l.known.detail = kept(market::get_app(port, market::next_id(), &l.id), &mut silent);
            l.known.logo = kept(market::get_logo(port, l.source, &l.id), &mut silent).flatten();
            l.known.described = true;
        }
        if selected && !l.known.judged {
            let release = market::get_release(port, market::next_id(), &l.id);
            l.known.release = kept(release, &mut silent);
            l.known.price = kept(market::get_price(port, market::next_id(), &l.id), &mut silent);
        }
        if selected {
            let readiness = market::install_ready(port, market::next_id(), &l.id);
            l.known.readiness = kept(readiness, &mut silent);
            l.known.judged = true;
            recheck(&mut l.known, now);
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
}
