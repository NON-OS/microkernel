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

//! What the pointer can be over. Each target is named from the layout pass
//! its painter draws from, so the painter, the click and the hover agree on
//! what lies at a point.

use super::header_slots::HeadHit;

/// A control or a row, by where its painter's layout pass keeps it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// A sidebar row with somewhere to go, by its index in `side_rows`.
    Side(usize),
    /// A header control.
    Head(HeadHit),
    /// A column of the list's header strip, by its index in `list_cols`.
    Sort(usize),
    /// The check box on the list row of the entry at this index.
    Check(usize),
    /// The list row of the entry at this index.
    Row(usize),
    /// The grid card of the entry at this index.
    Cell(usize),
    /// A selection band action, by its index in `sel_model::ACTIONS`.
    Band(usize),
    /// An info panel tile, by its index in `info_quick::QUICKS`.
    Quick(usize),
    /// The remove box of the info panel's tag at this index.
    Untag(usize),
    /// The info panel's add tag chip.
    AddTag,
    /// The info panel's permissions toggle.
    Perms,
    /// A Home category card, by its index in `home_cats::CATS`.
    Cat(usize),
    /// A Home storage card, by its index in `home_stor_geom::STOR_PLACES`.
    Stor(usize),
    /// A chip on a stacked screen, by its index in the chip row.
    Chip(usize),
    /// A row of a stacked screen, by its index in that screen's lines.
    Line(usize),
}
