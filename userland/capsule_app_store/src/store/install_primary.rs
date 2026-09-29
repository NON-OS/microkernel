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

//! Enter does the next sensible thing for the selected listing: install it,
//! wait while it installs, or open it once it has.

use super::install::{ask, open, Asked};
use super::progress::Progress;
use super::state::State;

/// What Enter means for the selected listing, as far as it has got.
pub fn primary(state: &mut State) -> Option<Asked> {
    let progress = state.current().map(|l| l.progress)?;
    match progress {
        Progress::Installed => Some(open(state)),
        p if p.pending() => None,
        _ => {
            let asked = ask(state);
            if asked == Asked::Queued {
                if let Some(l) = state.current_mut() {
                    l.progress = Progress::Queued;
                }
            }
            Some(asked)
        }
    }
}
