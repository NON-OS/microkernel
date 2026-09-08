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

//! The figure as typed, and what a screen may ask about it.
//!
//! Stake and swap each grew their own copy of this arithmetic, identical down to
//! the variable names, and send got a third that was not the same at all: it held
//! thousandths of an ether in a `u32` and had no decimal point, so the most a
//! reader could say was "0.001". An exact 0.0005 could not be entered at all. A
//! wallet that cannot spell the amount in your account is not finished.
//!
//! The digits are kept exactly as typed, as one integer with a count of how many
//! fell after the point, and scaled only when the chain asks. Nothing rounds on
//! the way in, so the amount signed is the amount shown.

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Amount {
    /// The digits typed, as one integer with the point taken out.
    pub(super) value: u128,
    /// How many keys were pressed, so a correction knows what to take back and a
    /// fresh figure knows it is replacing rather than extending.
    pub(super) typed: u32,
    /// How many of those digits fell after the point.
    pub(super) places: u32,
    pub(super) point: bool,
}

impl Amount {
    pub const fn new() -> Self {
        Amount { value: 0, typed: 0, places: 0, point: false }
    }

    /// The figure as typed, point removed. Only a formatter should need this.
    pub fn raw(&self) -> u128 {
        self.value
    }

    pub fn places(&self) -> u32 {
        self.places
    }

    pub fn point_started(&self) -> bool {
        self.point
    }

    /// True once anything at all has been typed, which is not the same as the
    /// figure being zero: "0.00" is typed and an untouched field is not. A screen
    /// that greys out its sign button has to tell them apart.
    pub fn typed_anything(&self) -> bool {
        self.typed > 0
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}
