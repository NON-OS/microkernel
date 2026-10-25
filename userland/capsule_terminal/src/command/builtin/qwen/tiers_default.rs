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

//! The line that says a default tier runs, and why.

use alloc::format;
use alloc::string::String;

use super::super::default_tier::Source;

/* The line that says a default runs, and why; None for the person's choice. */
pub fn default_line(tier: &str, source: Source) -> Option<String> {
    let why = match source {
        Source::Chosen => return None,
        Source::Stick => "the tier on the release stick, which installs offline",
        Source::Largest => "the largest tier this machine's memory runs",
        Source::Smallest => "the smallest tier, though none fits this machine's memory",
    };
    Some(format!(
        "qwen: no tier chosen at setup or in Settings; running the default, {tier}: {why}"
    ))
}
