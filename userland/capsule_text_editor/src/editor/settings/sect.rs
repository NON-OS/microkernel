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

//! The shape every section shares: a card headed by the section's name,
//! holding a row per setting. One table per section is all a new panel
//! costs.

use super::sects_a::{APPEARANCE, EDITING};

/// A row's control. Every kind has something behind it: the dropdowns the
/// panel used to draw had nothing behind them to open, and a control that
/// cannot be operated is not a disabled control, it is a picture of one.
#[derive(Clone, Copy)]
pub(super) enum Ctl {
    /// A switch over a bit of the section's state.
    Toggle(u32),
    /// The name of the theme in force; a press steps to the next one.
    Theme,
}

pub(super) struct Section {
    pub head: &'static str,
    pub rows: &'static [(&'static str, Ctl)],
}

pub(super) fn section(nav: usize) -> Option<&'static Section> {
    match nav {
        0 => Some(&EDITING),
        1 => Some(&APPEARANCE),
        _ => None,
    }
}
