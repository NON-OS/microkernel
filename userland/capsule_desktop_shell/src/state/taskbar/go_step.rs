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

//! What a menubar item that names an app does. Each click on Go > Terminal
//! used to start another window, up to the app's limit, while the one already
//! open (perhaps minimised) stayed where it was. An app with a window open is
//! brought forward instead, restored if it was minimised, as its dock tile
//! does; only an app with none opens one.

use super::types::TaskbarState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoStep {
    /// Raise app `index`'s window; open one if that finds none after all.
    Focus(usize),
    /// Open a new window.
    Open,
}

/// The step for the app at `index` in the launcher table, if it is there.
pub fn go_step(state: &TaskbarState, index: Option<usize>) -> GoStep {
    match index {
        Some(i) if state.open.get(i).copied().unwrap_or(false) => GoStep::Focus(i),
        _ => GoStep::Open,
    }
}
