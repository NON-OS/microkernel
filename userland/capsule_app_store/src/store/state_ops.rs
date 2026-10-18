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

//! Moving through the catalogue.

use alloc::vec::Vec;

use super::model_weights::MODEL_WEIGHTS;
use super::state::State;
use super::tier_fit::{order, shown};

#[path = "state_moving.rs"]
mod state_moving;
#[path = "state_new.rs"]
mod state_new;

impl State {
    /// Indices into `listings` that the current tab and the search show,
    /// less the Qwen tiers that do not fit this machine, the tiers last and
    /// smallest first (`tier_fit`). A description counts once the market
    /// has said it (`fill`); the filter itself reads only what is held, so
    /// typing asks the market nothing.
    pub fn visible(&self) -> Vec<usize> {
        let mut out: Vec<usize> = self.matching().filter(|&i| self.fits(i)).collect();
        out.sort_by_key(|&i| order(&self.listings[i].id, MODEL_WEIGHTS));
        out
    }

    /// How many tiers the tab and the search would show but this machine
    /// has too little memory for.
    pub fn hidden(&self) -> usize {
        self.matching().filter(|&i| !self.fits(i)).count()
    }

    fn matching(&self) -> impl Iterator<Item = usize> + '_ {
        self.listings.iter().enumerate().filter_map(|(i, l)| {
            let description = l.known.detail.as_ref().map(|d| d.description.as_slice());
            let kept = self.tab.accepts(l.source) && self.search.matches(&l.name, description);
            kept.then_some(i)
        })
    }

    fn fits(&self, i: usize) -> bool {
        shown(&self.listings[i].id, MODEL_WEIGHTS, self.memory, self.room)
    }
}
