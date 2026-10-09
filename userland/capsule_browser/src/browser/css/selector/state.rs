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

/* Form-control conditions, read from the element's attributes and its place
 * in the tree. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormState {
    Enabled,
    Disabled,
    Checked,
    Default,
    Indeterminate,
    Required,
    Optional,
    ReadOnly,
    ReadWrite,
    PlaceholderShown,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
}

/* Conditions on what the user is doing, read from the matcher's element
 * state: the hovered, pressed, focused and target elements. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserState {
    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    Target,
}
