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

//! The shape every non-General section shares: a heading over a card of rows,
//! each row carrying either a live switch or a dimmed dropdown. One table per
//! section is all a new panel costs, and the card body is the only geometry
//! that varies with the row count.

use super::card::{card_rect, ROW_H};
use super::sects_a::EDITING;

/// A row's control. One kind, because a switch is the only control the panel
/// can honour: the dropdowns it used to draw had nothing behind them to open,
/// and a control that cannot be operated is not a disabled control, it is a
/// picture of one.
#[derive(Clone, Copy)]
pub(super) enum Ctl {
    Toggle(u32),
}

pub(super) struct Section {
    pub head: &'static str,
    pub rows: &'static [(&'static str, Ctl)],
}

pub(super) fn sect_rect(width: u32, rows: usize) -> (u32, u32, u32, u32) {
    let (x, y, w, _) = card_rect(width);
    (x, y, w, ROW_H * rows as u32)
}

pub(super) fn section(nav: usize) -> Option<&'static Section> {
    match nav {
        1 => Some(&EDITING),
        _ => None,
    }
}
