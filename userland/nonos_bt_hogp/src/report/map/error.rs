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

//! Why a Report Map was not used. Every one of these leaves the service in
//! boot mode, which needs no map at all.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapError {
    /// The map ends inside an item.
    Short,
    /// A long item, or a reserved item type or tag: nothing this host
    /// decodes needs one, and guessing at its meaning would be worse.
    Unknown,
    /// An item's value is outside what the item allows.
    Value,
    /// A collection, push, field, usage or report count past this host's
    /// bounds, or a report longer than it reads.
    TooMany,
    /// Collections or pushes that do not pair, a Usage Minimum with no
    /// Maximum, or numbered and unnumbered reports mixed.
    Shape,
}
