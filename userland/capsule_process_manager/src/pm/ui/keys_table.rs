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

//! Narrowing the table, and acting on what is selected.

use super::keys::{Act, Binding};
use super::keys_filter::FILTERS;
use super::keys_group::Group;
use super::keys_nav::NAV;
use super::keys_sort::SORT;

pub const ACTS: &[Binding] = &[
    Binding {
        key: b"a",
        codes: &[0x41, 0x61],
        label: b"every process",
        group: Group::Filter,
        act: Act::FilterAll,
    },
    Binding {
        key: b"e",
        codes: &[0x45, 0x65],
        label: b"only those holding sensitive authority",
        group: Group::Filter,
        act: Act::FilterElevated,
    },
    Binding {
        key: b"t",
        codes: &[0x54, 0x74],
        label: b"only core processes that cannot be ended",
        group: Group::Filter,
        act: Act::FilterProtected,
    },
    Binding {
        key: b"g",
        codes: &[0x47, 0x67],
        label: b"only those the monitor flagged",
        group: Group::Filter,
        act: Act::FilterFlagged,
    },
];

/// Every binding, in the order the overlay reads them. Two slices rather than
/// one because a single list ran past the file limit, and a pair keeps the
/// grouping visible instead of hiding it in the middle of a long literal.
pub const BINDINGS: [&[Binding]; 4] = [NAV, SORT, FILTERS, ACTS];

/// Walk them all.
pub fn all() -> impl Iterator<Item = &'static Binding> {
    BINDINGS.iter().flat_map(|s| s.iter())
}

/// The action a key code asks for, or nothing. Digits are handled by the caller
/// because the screen they pick is positional rather than a fixed binding.
pub fn act_for(code: u32) -> Option<Act> {
    all().find(|b| b.codes.contains(&code)).map(|b| b.act)
}
