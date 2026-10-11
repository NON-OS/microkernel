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

//! `d`: a Qwen tier installed with its model downloaded direct.

use nonos_libc::mk_app_install;

use super::asked::Asked;
use super::progress::Progress;
use super::state::State;

/// `d`: ask for the selected Qwen tier to be installed with its model
/// downloaded over a direct connection, for this install only. Offered
/// only where a download through an anonymity network would be large
/// (`route_offer::direct_offered`); init reads the release word `@direct`.
pub fn direct(state: &mut State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    if !direct_offered_for(state, listing) {
        return Asked::NoDirect;
    }
    if !listing.ready {
        return Asked::NotReady;
    }
    let asked = match mk_app_install(&listing.id, DIRECT_RELEASE) {
        0 => Asked::QueuedDirect,
        -16 => Asked::Busy,
        errno => Asked::Refused(errno),
    };
    if asked == Asked::QueuedDirect {
        if let Some(l) = state.current_mut() {
            l.progress = Progress::Queued;
            l.removing = false;
        }
    }
    asked
}

/// The release word init reads as a direct download (linux_jobs/direct.rs).
const DIRECT_RELEASE: &[u8] = b"@direct";

/// Whether `d` would ask for a direct download of `l`'s model.
pub fn direct_offered_for(state: &State, l: &super::listing::Listing) -> bool {
    let left = super::tier_fit::tier_of(&l.id)
        .and_then(|t| super::tier_fit::weight(t, super::model_weights::MODEL_WEIGHTS));
    left.is_some_and(|left| {
        let why = match l.progress {
            super::progress::Progress::Failed(w) => Some(w),
            _ => None,
        };
        super::route_offer::direct_offered(state.route, left, l.progress.uninstalled(), why)
    })
}
