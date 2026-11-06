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

//! Each prompt's two buttons as the shared `Dialog` draws them, left to
//! right: the one that acts, then the one that leaves things as they are.
//! A press on the first answers Act, so it is index 0 of the dialog's hit.

use nonos_toolkit::widgets::Kind;

/// A prompt's buttons, as a label and a kind each.
pub type Buttons = [(&'static str, Kind); 2];

/// The app and package consents.
pub const APPROVE: Buttons = [("Approve", Kind::Primary), ("Cancel", Kind::Secondary)];

/// The live session's offer to install NONOS.
pub const INSTALL: Buttons = [("Install NONOS", Kind::Primary), ("Not now", Kind::Secondary)];

/// A desktop Delete: the button that loses the item is a `Danger` one, so
/// it does not read as the safe one.
pub const DELETE: Buttons = [("Delete", Kind::Danger), ("Cancel", Kind::Secondary)];
