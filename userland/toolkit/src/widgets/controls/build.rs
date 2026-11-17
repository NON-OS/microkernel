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

//! The controls of a surface: the pointer and how a click acts.

use alloc::vec::Vec;

use super::targets::Target;
use crate::widgets::Pointer;

/// The pointer over one surface and the hit boxes of its controls, each named
/// by an action `A`. The app replaces the boxes wherever it lays them out, so
/// the boxes a click tests are the boxes the paint drew.
#[derive(Clone, Debug)]
pub struct Controls<A> {
    pub(super) pointer: Pointer,
    pub(super) targets: Vec<Target<A>>,
    pub(super) held: Option<A>,
    pub(super) on_press: bool,
}

impl<A: Copy + PartialEq> Controls<A> {
    /// Controls that act on release over the control the press began on, so a
    /// press dragged off clicks nothing. This is the default.
    pub fn new() -> Self {
        Controls { pointer: Pointer::new(), targets: Vec::new(), held: None, on_press: false }
    }

    /// Controls that act the moment a press lands on a live control.
    pub fn acting_on_press() -> Self {
        Controls { on_press: true, ..Self::new() }
    }
}

impl<A: Copy + PartialEq> Default for Controls<A> {
    fn default() -> Self {
        Self::new()
    }
}
