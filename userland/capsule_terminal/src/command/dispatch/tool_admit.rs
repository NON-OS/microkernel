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

//! Which redirects a baked tool can be given, decided from the words alone
//! before anything is read, written or started.

use alloc::vec::Vec;

use super::redirect::{Plan, Source};

/// Whether the tool `name` hears a lone 0x04 as the end of its input. Only
/// the Linux personality does; a NONOS tool's stdin has no end yet.
pub fn hears_end_of_input(name: &[u8]) -> bool {
    name == b"linux"
}

/// `Ok` when the tool `name` can run as `plan` asks, or the one line that
/// says why it cannot.
pub fn admit(name: &[u8], plan: &Plan<'_>) -> Result<(), Vec<u8>> {
    if plan.words.contains(&b"|".as_slice()) {
        if hears_end_of_input(name) {
            return Err(b"linux: a pipe into or out of a Linux program is done inside it; use: linux sh -c 'prog | grep x'".to_vec());
        }
        return Err([
            name,
            b": a tool's output does not feed a pipe; use > file, then the filter on the file"
                .as_slice(),
        ]
        .concat());
    }
    if plan.input != Source::Terminal && !hears_end_of_input(name) {
        return Err([
            name,
            b": < is not taken: this program has no end of input to be given after a file"
                .as_slice(),
        ]
        .concat());
    }
    Ok(())
}
