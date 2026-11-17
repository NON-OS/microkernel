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

//! The Resonare interface. `metrics` and `geometry` are the design tokens,
//! and every colour is a toolkit theme role; the controls are the toolkit's
//! shared widgets, with the player's own looks on them in `drawn` and `marks`;
//! `art`, `icon` and
//! `widget` are the drawn vocabulary; `shell` and `screen` compose them; `hit`
//! maps a pointer back onto whatever those painters put on the glass, through
//! the table in `targets` that the hover reads too.

pub mod art;
pub mod control;
mod drawn;
pub mod event;
mod fmt;
pub mod frame;
pub mod geometry;
pub mod hit;
mod hit_screen;
pub mod hover;
pub mod icon;
mod marks;
pub mod metrics;
pub mod screen;
pub mod search_key;
pub mod shortcut;
pub mod shell;
pub mod shown;
mod sprite;
pub mod state;
pub mod target;
mod targets;
mod targets_page;
pub mod view;
pub mod widget;

pub use control::Control;
pub use frame::{Frame, Scene};
pub use hit::{hit, shown, Action, Lists};
pub use state::{UiState, View};
