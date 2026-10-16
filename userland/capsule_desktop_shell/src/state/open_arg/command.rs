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

//! The command line a tool tile hands the Terminal: run as typed, or left
//! on the prompt for its arguments.

use alloc::string::String;

/// Hand-synced with the Terminal's term/handed/parse.rs.
pub const RUN: &str = "run:";

pub const TYPE: &str = "type:";

/// Tools whose run with no arguments prints their usage: pastel shows its
/// help. Every other tool does nothing useful bare, so its tile types the name
/// and leaves the cursor after it for the arguments: grex stops on a missing
/// argument; jsonxf, huniq and csview read stdin when given no file and would
/// sit waiting on the keyboard; and tokei and dotenv-linter work on the
/// current directory, which for a tool is always `/` (nonos-std getcwd), so
/// tokei would count the whole file system. A tool added to apps.list is
/// typed, not run, until it is named here.
const RUNS_BARE: [&[u8]; 1] = [b"pastel"];

pub fn runs_bare(tool: &[u8]) -> bool {
    RUNS_BARE.contains(&tool)
}

/// What a tool tile hands the Terminal, or None for a name that is not a
/// plain tool name (the generated table only holds such names).
pub fn tool_command(tool: &[u8]) -> Option<String> {
    let name = core::str::from_utf8(tool).ok()?;
    let plain = name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    if name.is_empty() || !plain {
        return None;
    }
    let mut line = String::with_capacity(TYPE.len() + name.len() + 1);
    if runs_bare(tool) {
        line.push_str(RUN);
        line.push_str(name);
    } else {
        line.push_str(TYPE);
        line.push_str(name);
        line.push(' ');
    }
    Some(line)
}
