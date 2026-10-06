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

use crate::calc::fixed::Fixed;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Length,
    Weight,
    Temperature,
    Data,
}

// No currency: the calculator has no source of live exchange rates, and a
// table of fixed ones would give a wrong answer that looks right.
pub const CATEGORIES: [Category; 4] =
    [Category::Length, Category::Weight, Category::Temperature, Category::Data];

pub struct Unit {
    pub name: &'static str,
    pub num: Fixed,
    pub den: Fixed,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Length => "Length",
            Category::Weight => "Weight",
            Category::Temperature => "Temperature",
            Category::Data => "Data",
        }
    }
}

pub fn list(cat: Category) -> &'static [Unit] {
    match cat {
        Category::Length => &super::length::LENGTH,
        Category::Weight => &super::mass::WEIGHT,
        Category::Temperature => &super::temp::TEMPERATURE,
        Category::Data => &super::data::DATA,
    }
}
