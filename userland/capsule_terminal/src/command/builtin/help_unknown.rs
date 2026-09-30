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

//! What `help <name>` says when the name has no usage entry.

use crate::command::output::Output;

/// A name with no entry is not necessarily a name with no command: the chain
/// set carries its own, and tools document themselves. Say which door to try
/// rather than only that this one is shut.
pub(super) fn unknown(out: &mut Output<'_>, name: &[u8]) -> bool {
    if super::tool::is_tool(name) {
        out.writeln(b"an installed tool; run it with --help for its own options");
        return true;
    }
    if crate::event::complete::is_command_name(name) {
        out.writeln(b"a shell command with no usage entry yet; 'help' lists the groups");
        return true;
    }
    let _ = name;
    out.writeln(b"no such command; 'help' lists what there is");
    false
}
