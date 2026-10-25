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

//! What a search of the takeover model found (L79).

use super::types::Stage;
use super::world::{code, Page, END};
use super::world_state::World;

/// Worlds seen and ends reached, the invariants broken, the events, and the
/// ends with G1 ready, with bootfb's pages on the sink, and with G1 refused.
#[derive(Default, Debug, Clone, Copy)]
pub(super) struct Found {
    pub worlds: usize,
    pub ends: usize,
    pub broken: u16,
    pub events: u16,
    pub ready: usize,
    pub taken: usize,
    pub failed: usize,
}

impl Found {
    /// A world where nothing can move: held to the end's invariants, counted.
    pub(super) fn ended(&mut self, w: &World) {
        self.broken |= w.end_broken();
        self.ends += 1;
        self.ready += usize::from(w.gpu[0] == Some(code(Stage::Ready)));
        self.taken += usize::from(w.page.contains(&Page::Sink));
        self.failed += usize::from(w.gpu[0].is_none() && w.pc[0] == END);
    }
}
