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

//! What the exit of the next general circuit must take.

extern crate alloc;

use alloc::vec::Vec;

use crate::path::{draw_path_through, Exits, Relay, Weights};

/// The port a general circuit is drawn for when no stream waits on another:
/// the web ports, by the rule they always had.
pub(super) const WEB: u16 = 443;

/// What the exit of the next general circuit must take.
#[derive(Clone, Copy)]
pub struct Want {
    /// The port of the stream it is for, or 443, the web rule, for none.
    pub port: u16,
    /// A stream waits on the port. Its circuit is never the warm one, which
    /// takes installs only (warm_rule.rs).
    pub waited: bool,
}

impl Want {
    /// A path through `guard` whose exit `exits` says takes the port.
    pub fn draw(
        self,
        guard: &Relay,
        relays: &[Relay],
        weights: &Weights,
        exits: &Exits,
    ) -> Option<Vec<Relay>> {
        draw_path_through(guard, relays, weights, |exit| exits.takes(exit, self.port))
    }
}
