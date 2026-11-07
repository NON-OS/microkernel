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

//! The GUI terminal's output sink, seen through the library's `ShellOut` trait.
//! A builtin that takes `&mut dyn nonos_shell::out::ShellOut` writes to the
//! scrollback through this, exactly as it does through `Output::writeln` today;
//! a headless nsh session plugs in its own `ShellOut` over its stream instead.

use super::types::Output;
use nonos_shell::out::ShellOut;

impl ShellOut for Output<'_> {
    fn writeln(&mut self, line: &[u8]) {
        Output::writeln(self, line)
    }
}
