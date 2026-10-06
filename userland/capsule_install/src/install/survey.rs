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

/*
 * The disk list: made when the disks screen opens, made again when a
 * person comes back to it from a stopped install, on R, and on its own
 * while it has nothing to install to or a row for a missing driver. The
 * disk that was selected stays selected when it is still there.
 */

use nonos_blk_client::survey;
use nonos_libc::mk_time_millis;

use super::rescan::due;
use super::state::{Looked, Screen, State};

pub fn look(state: &mut State) {
    let keep = state.selected_disk().map(|d| (d.driver, d.instance));
    let started = mk_time_millis();
    let found = survey();
    state.raid = found.raid_hides_disks();
    state.incomplete = found.incomplete();
    state.disks = found.disks;
    let ended = mk_time_millis();
    state.looked = Some(Looked { ended, took: ended.saturating_sub(started) });
    state.selected = keep
        .and_then(|(driver, instance)| {
            state.disks.iter().position(|d| d.driver == driver && d.instance == instance)
        })
        .unwrap_or(0);
}

/// The disks screen is up and a later look may find more.
pub fn watching(state: &State) -> bool {
    state.screen == Screen::Disks && state.incomplete
}

/// Look again when watching and due. True when it looked, so the screen
/// is painted with what it found.
pub fn poll(state: &mut State) -> bool {
    if !watching(state) {
        return false;
    }
    if let Some(l) = state.looked {
        if !due(mk_time_millis(), l.ended, l.took) {
            return false;
        }
    }
    look(state);
    true
}
