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

use alloc::string::String;

use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::Pill;

use crate::browser::shell::groups;
use crate::browser::shell::Shell;
use crate::browser::ui::grid_geom::chip_rect;
use crate::browser::ui::rect::Rect;

/// What the chip of a tab in `group` says, and the group's category; None
/// for no group.
pub fn look(shell: &Shell, group: Option<u8>) -> (String, Option<Category>) {
    match group.and_then(|id| groups::find(&shell.groups, id)) {
        Some(g) => (g.name.clone(), Some(Category::new(g.color as usize))),
        None => (String::from("No group"), None),
    }
}

/// The chip: a pill filled in the group's colour, or a plain one reading
/// "No group".
pub fn pill(label: &str, k: Option<Category>) -> Pill<'_> {
    let p = Pill::new(label);
    k.map_or(p, |k| p.filled(k))
}

/// Where the chip `p` sits on its card `card`.
pub fn rect(card: Rect, p: &Pill) -> Rect {
    let (w, h) = p.size();
    chip_rect(card, w as i32, h as i32)
}
