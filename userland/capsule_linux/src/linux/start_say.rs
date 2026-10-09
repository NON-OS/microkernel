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

//! When the personality keeps quiet about a run.

/// A line that only narrates a healthy run. A terminal's run shows the
/// person what went wrong, not the personality's own progress: a tier run on
/// a terminal keeps it here, and the terminal's `linux` command in `note`.
pub(crate) fn routine(line: &[u8]) {
    let cli = matches!(super::request::run_request(), Some((_, super::run_mode::Mode::Cli)));
    if !cli && !super::console::private() {
        super::say::note(line);
    }
}
