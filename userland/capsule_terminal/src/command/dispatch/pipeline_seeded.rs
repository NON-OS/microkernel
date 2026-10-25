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

//! A chain of filters over lines a `<` file fed it, with no command before them.

use alloc::vec::Vec;

use super::super::filter::apply;
use super::split_stages;

// Apply a `a | b | c` filter chain to pre-seeded input lines with no
// producer command: used when `< file` supplies the input instead of a
// leading command's captured output.
pub(in super::super) fn run_filters(seed: Vec<Vec<u8>>, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut lines = seed;
    for seg in &split_stages(args) {
        lines = apply(seg, lines);
    }
    lines
}
