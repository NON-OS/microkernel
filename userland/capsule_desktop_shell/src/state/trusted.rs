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

//! The shell's half of the trusted path: which band its chrome is in, the
//! secure attention chord's state, the power-off confirmation and the display
//! input menu.

use super::attention::Attention;
use super::input_menu::InputMenu;
use super::power_off::PowerOff;

#[derive(Clone, Copy, Debug, Default)]
pub struct Trusted {
    /// Whether the chrome is in the trusted band now.
    pub up: bool,
    pub attention: Attention,
    /// The power-off confirmation, which only the router can answer.
    pub power_off: PowerOff,
    /// The display input menu, which only the router can answer.
    pub input_menu: InputMenu,
}
