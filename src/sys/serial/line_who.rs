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

//! `Line::who`: a process's name, as a console may show it (who.rs).

use super::line::Line;
use super::who::shown;

impl Line {
    /// Append the process name `name`, a Linux guest's left out unless the
    /// build has capsule serial output.
    pub fn who(&mut self, name: &[u8]) -> &mut Self {
        self.str(shown(name, cfg!(feature = "capsule-serial-debug")))
    }
}
