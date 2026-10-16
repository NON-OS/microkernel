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

//! The rows of the Accessibility page. They are not policy fields: the
//! compositor holds them for each display, and Settings reads and sets them
//! through the shell (`settings/access`).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Knob {
    /// Which display the rows below are for.
    Output,
    Filter,
    Contrast,
    Magnify,
    Follow,
}

impl Knob {
    pub fn label(self) -> &'static str {
        match self {
            Knob::Output => "Display",
            Knob::Filter => "Colour filter",
            Knob::Contrast => "High contrast",
            Knob::Magnify => "Magnification",
            Knob::Follow => "Magnifier follows",
        }
    }
}
