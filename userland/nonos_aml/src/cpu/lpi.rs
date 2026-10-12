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

//! _LPI (ACPI 6.5, 8.4.4.3): the low-power idle states of one level of the
//! processor hierarchy, a processor's own or a processor container's
//! (ACPI0010) that every processor under it must agree to enter. The
//! package is a revision, the level's id, a count, then one package per
//! state. The field order is recalled from the specification and was not
//! checked against its text here (unverified).

use super::lpi_state::state;
use super::lpi_types::Lpi;
use crate::object::Object;
use alloc::vec::Vec;

/// More than any level lists; a level that claims more is refused whole,
/// because a child names its parent's states by position.
pub const MAX_LPI_STATES: usize = 8;
/// The state flags' bit 0: firmware has enabled the state.
pub const LPI_ENABLED: u32 = 1;

/// The level, or None when it is not a package of the shape above, its
/// count is not the states it holds, or any state is malformed: a state
/// left out would shift the positions the level below refers to.
pub fn parse_lpi(o: &Object) -> Option<Lpi> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    let int = |i: usize| match e.get(i) {
        Some(Object::Integer(v)) => Some(*v),
        _ => None,
    };
    let (revision, level, count) = (int(0)?, int(1)?, int(2)?);
    let count = usize::try_from(count).ok().filter(|c| *c <= MAX_LPI_STATES)?;
    if e.len() != 3 + count {
        return None;
    }
    let states = e[3..].iter().map(state).collect::<Option<Vec<_>>>()?;
    Some(Lpi { revision, level, states })
}
