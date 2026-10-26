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

//! Which moves are refused before anything runs (ACPI 6.5, 7.3): D1 and
//! D2 exist only on a device that defines _PSx or _PRx for them, and a
//! device in a low-power state comes back to D0 before going elsewhere.

use super::state::DState;
use crate::load::Aml;
use crate::namespace::Id;

#[derive(Debug)]
pub enum MoveError {
    /// D1 or D2 on a device with neither _PSx nor _PRx for it: the device
    /// has no such state.
    NoSuchState,
    /// Between two low-power states: the device goes back to D0 first.
    NotFromD0,
    Fault(crate::exec::Fault),
}

pub(super) fn refused(aml: &Aml, dev: Id, from: DState, to: DState) -> Option<MoveError> {
    let defines = |s| aml.ns.child(dev, s).is_some();
    let shallow = matches!(to, DState::D1 | DState::D2);
    if shallow && !(defines(to.ps()) || to.pr().is_some_and(defines)) {
        return Some(MoveError::NoSuchState);
    }
    (from != DState::D0 && to != DState::D0 && from != to).then_some(MoveError::NotFromD0)
}
