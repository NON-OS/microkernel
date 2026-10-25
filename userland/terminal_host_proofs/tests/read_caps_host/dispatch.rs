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

//! The dispatcher as the `<` input sees it: a stand-in for the filter input's
//! line split, and the run module holding only run_source.

pub mod filter {
    pub mod input {
        // Replaces filter/input_lines.rs's lines_of, which splits a file into
        // a filter's lines: here the bytes it is handed come back as one
        // line, so a test sees exactly what read_input passed on.
        pub fn lines_of(bytes: &[u8]) -> Vec<Vec<u8>> {
            vec![bytes.to_vec()]
        }
    }
}

#[path = "dispatch_run.rs"]
mod run;
