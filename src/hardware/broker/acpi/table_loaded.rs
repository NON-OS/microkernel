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

//! A table AML loads at run time (F-93): Load from an OperationRegion or a
//! buffer, or LoadTable. The capsule hands the exact bytes it will load;
//! they are checked whole (loaded_rule) and measured into the same list as
//! the firmware's tables, so the attested root moves; refused when the list
//! is full, and the capsule then does not load them.

use super::loaded_rule::{checked, Admit, LOADED};
use super::table_measure::Measured;
use super::table_measured::add;
use super::table_sweep::sweep_once;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadedError {
    /// Not one whole table: short, a wrong length, or a bad checksum.
    Malformed,
    /// The list is full; the registry is marked incomplete.
    Full,
}

/// A table AML is about to load: measured first, refused when the list is
/// full, so nothing loads that the attested root does not cover.
pub fn record_loaded(bytes: &[u8]) -> Result<(), LoadedError> {
    sweep_once(add);
    let sig = checked(bytes).ok_or(LoadedError::Malformed)?;
    match add(Measured::of(sig, LOADED, bytes)) {
        Admit::Full => Err(LoadedError::Full),
        Admit::Added | Admit::Known => Ok(()),
    }
}
