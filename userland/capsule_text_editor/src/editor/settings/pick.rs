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

//! What a press on the Settings screen does: a rail row selects its section,
//! a switch row flips its setting and the theme row steps to the next theme.

use super::sect::{section, Ctl};
use super::sect_state::flip_sect;
use super::state::{select_nav, state};
use crate::editor::theme;

/// A control on the Settings screen: rail row `i`, or row `i` of the section
/// on show.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Pick {
    Nav(usize),
    Row(usize),
}

/// Carries out `pick`, and says whether anything changed.
pub(super) fn act(pick: Pick) -> bool {
    let nav = state().nav;
    let row = |i: usize| section(nav).and_then(|s| s.rows.get(i)).map(|r| r.1);
    match pick {
        Pick::Nav(i) => select_nav(i),
        Pick::Row(i) => match row(i) {
            Some(Ctl::Toggle(bit)) => {
                flip_sect(nav, bit);
                true
            }
            Some(Ctl::Theme) => {
                theme::cycle();
                true
            }
            None => false,
        },
    }
}
