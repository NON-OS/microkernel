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

/* The parent's content box as the percentages of an in-flow child see it.
 * Absolutely positioned boxes do not read it: the positioned ancestor that
 * contains them places them against its own padding box once its height is
 * final (place_out). */
#[derive(Clone, Copy)]
pub(crate) struct Containing {
    /* Content width: the base of a relative box's left/right percentages. */
    pub w: i32,
    /* Definite content height, None when it sizes to content. Percentage
     * heights resolve only against a definite value. */
    pub h: Option<i32>,
}
