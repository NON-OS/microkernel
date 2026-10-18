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

//! The install that is moving, and what the fetcher said of a tier's download.

use super::super::listing::Listing;
use super::super::model_weights::MODEL_WEIGHTS;
use super::super::state::State;

impl State {
    /// What the fetcher said of `l`'s download, when `l` is the tier
    /// installing and the answer is its: the tier whose files sum to it.
    /// Another listing whose install or uninstall is moving, which a queued
    /// one waits behind: the personality runs one installer at a time.
    pub fn ahead_of(&self, l: &Listing) -> Option<&Listing> {
        use super::super::progress::Progress;
        self.listings.iter().find(|o| {
            o.id != l.id && matches!(o.progress, Progress::Installing | Progress::Removing)
        })
    }

    pub fn fetch_of(&self, l: &Listing) -> Option<crate::status_wire::Status> {
        let s = self.fetch?;
        let tier = super::super::tier_fit::tier_of(&l.id)?;
        // Starting has no total yet; one install runs at a time, so it is
        // the tier installing.
        let mine = s.stage == crate::status_wire::STARTING
            || super::super::tier_fit::weight(tier, MODEL_WEIGHTS) == Some(s.total);
        (l.progress == super::super::progress::Progress::Installing && mine).then_some(s)
    }
}
