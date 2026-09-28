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

//! The tools line of `help`.

use crate::command::output::Output;

/// The installed crates.io programs, listed from the table that runs them.
///
/// These are ordinary published crates, built for this system and admitted by
/// the same spawn gate as everything else. They are worth naming here because
/// nothing else on screen says they exist, and a tool nobody can discover may
/// as well not be installed.
pub fn tools(out: &mut Output<'_>) {
    let list = super::tool_list::tool_list(super::tool::TOOLS);
    super::help::row(out, b"tools", &list, super::help_layout::GROUP_PAD);
}
