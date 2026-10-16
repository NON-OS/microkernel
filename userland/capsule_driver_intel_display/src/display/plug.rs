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

//! Whether an output's monitor is there, as the hot-plug poll last settled
//! it (`plug_poll`). Only an output with mode setting follows its line:
//! one without cannot be lit again once dark, and eDP's panel never leaves.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plug {
    /// Attached, and lit as the takeover or the last mode set left it.
    In,
    /// Pulled: the port dark, and nothing on it touched until it is back.
    Out,
    /// Attached again with its EDID read, dark until a mode set lights it;
    /// STATE says LOST meanwhile, so the wire server puts the mode back.
    Back,
}
