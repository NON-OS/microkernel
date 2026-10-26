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

//! The release an install finished as (ruling 7): the store asks for the
//! default release, so when one it watched moving reads Installed, the
//! release it knew then is the one installed. One installed before this
//! store session is not known, and its proof asks the default.

use super::super::listing::Listing;
use super::super::progress::Progress;
use super::proof_state::Proofs;

impl Proofs {
    /// Moves `l` to `now`, as the system said it; true when it changed.
    pub fn moved(&mut self, l: &mut Listing, now: Progress) -> bool {
        let finished = l.progress.pending() && now == Progress::Installed;
        let release = l.known.release.as_ref().map(|r| r.version.clone());
        if let Some(release) = release.filter(|_| finished) {
            self.installed.retain(|(id, _)| *id != l.id);
            self.installed.push((l.id.clone(), release));
        }
        let changed = now != l.progress;
        l.progress = now;
        changed
    }
}
