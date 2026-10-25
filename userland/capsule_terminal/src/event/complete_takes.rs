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

//! Which commands take another command's name as their argument.

/// Commands whose argument is another command's name rather than a path.
///
/// Without this, `help gr<Tab>` looks for a file called `gr`, finds nothing,
/// and appears broken. The completion a reader gets has to match what the
/// command actually takes, or Tab teaches them not to press it.
const TAKES_COMMAND: &[&[u8]] = &[b"help", b"commands", b"type", b"which"];

pub fn takes_command_argument(first: &[u8]) -> bool {
    TAKES_COMMAND.contains(&first)
}
