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

//! What a run says of the packs it could not load: one line for each
//! reason, with how many and the first, so a closure refused for one cause
//! (a machine that was not let run what it installs refuses every pack of
//! it) reads as one line where the person typed, not one a pack. Pure, for
//! the host proofs.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The lines for `refused` (each pack's path and why), a reason's line
/// where it first came.
pub fn lines(refused: &[(&[u8], &str)]) -> Vec<String> {
    // Each reason, the first pack it came with, and how many: few reasons,
    // so one pass over the packs.
    let mut by: Vec<(&str, &[u8], usize)> = Vec::new();
    for &(path, why) in refused {
        match by.iter_mut().find(|(w, _, _)| *w == why) {
            Some(seen) => seen.2 += 1,
            None => by.push((why, path, 1)),
        }
    }
    let line = |(why, path, n): (&str, &[u8], usize)| {
        let path = String::from_utf8_lossy(path);
        match n {
            1 => format!("[LINUX] pack {path} not served: {why}\n"),
            _ => format!("[LINUX] {n} packs not served: {why} ({path} and {} more)\n", n - 1),
        }
    };
    by.into_iter().map(line).collect()
}
