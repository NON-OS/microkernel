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

//! Getting the install ready once a disk is chosen, before the word is
//! typed: on a disk an update could keep, which row starts lit, then the
//! whole plan for that row (`plan.rs`), made again when the row changes.
//! Nothing is written.

use super::plan::plan_for;
use crate::install::source::Image;
use crate::install::state::{Choice, State};

pub fn prepare(state: &mut State) {
    let image = state.image.as_ref().map(Image::as_disk_image);
    let keep = state.selected_disk().and_then(|d| d.keep);
    state.keep = keep.zip(image).map(|(k, image)| k.for_image(&image));
    /*
     * The update row starts lit wherever it shows: when the data is ready
     * it is the default, and when it is not, the screen says how to get it
     * ready before a key can lead to an erase.
     */
    state.choice = if state.offers_update() { Choice::Update } else { Choice::Erase };
    replan(state);
}

/// Plan again for the row now lit. The old plan and its store go first,
/// so the heap never holds two.
pub fn replan(state: &mut State) {
    state.prepared = None;
    /* The update row of a disk not ready plans nothing: nothing starts. */
    if !state.on_update_row() || state.updating() {
        state.prepared = Some(plan_for(state));
    }
}
