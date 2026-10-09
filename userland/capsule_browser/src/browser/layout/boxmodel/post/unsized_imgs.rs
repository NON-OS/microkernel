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

use alloc::string::String;
use alloc::vec::Vec;

use super::super::layout_block::MAX_DEPTH;
use super::super::replaced_size::awaits_natural;
use super::super::tree::{BoxKind, BoxNode};

/// The sources of every image whose box waits on a natural size not yet
/// known, so the page lays out again when one arrives.
pub(crate) fn unsized_imgs(root: &BoxNode) -> Vec<String> {
    let mut out = Vec::new();
    walk(root, &mut out, 0);
    out
}

fn walk(n: &BoxNode, out: &mut Vec<String>, depth: u32) {
    if depth > MAX_DEPTH {
        return;
    }
    if let BoxKind::Image { src, .. } = &n.kind {
        if awaits_natural(n) && !out.contains(src) {
            out.push(src.clone());
        }
    }
    for c in &n.children {
        walk(c, out, depth + 1);
    }
}
