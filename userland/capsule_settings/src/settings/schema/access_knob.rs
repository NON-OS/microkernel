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

//! The rows the compositor holds, not policy fields: the Accessibility
//! page's for each display (`settings/access`), and the displays'
//! arrangement and scales on Appearance (`settings/screens`). Settings reads
//! and sets them through the shell.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Knob {
    /// Which display the rows below are for.
    Output,
    Filter,
    Contrast,
    Magnify,
    Follow,
    /// Mirrored or extended across the displays.
    Arrange,
    /// Which display the scale row below is for.
    Screen,
    Scale,
}

impl Knob {
    /// One of the displays' rows on Appearance, not Accessibility's.
    pub fn on_screens(self) -> bool {
        matches!(self, Knob::Arrange | Knob::Screen | Knob::Scale)
    }
}

impl Knob {
    pub fn label(self) -> &'static str {
        match self {
            Knob::Output => "Display",
            Knob::Filter => "Colour filter",
            Knob::Contrast => "High contrast",
            Knob::Magnify => "Magnification",
            Knob::Follow => "Magnifier follows",
            Knob::Arrange => "Arrangement",
            Knob::Screen => "Display",
            Knob::Scale => "Scale",
        }
    }
}
