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

use super::state::Builder;

impl Builder {
    /// Keep the per-name count of open elements in step with the stack.
    pub(in super::super) fn count_name(&mut self, id: usize, add: bool) {
        let slot = &mut self.open_names[bucket(&self.dom.nodes[id].tag)];
        *slot = if add { slot.saturating_add(1) } else { slot.saturating_sub(1) };
    }

    /// False only when no open element has this name. An end tag or scope
    /// check for a name nothing opened is answered here instead of walking
    /// the stack, which is what made a page of unmatched end tags quadratic.
    /// Names share buckets, so true means "maybe": the walk decides.
    pub(in super::super) fn maybe_open(&self, name: &str) -> bool {
        self.open_names[bucket(name)] != 0
    }
}

/// FNV-1a over a tag name with ASCII case folded, so an SVG element's
/// `clipPath` shares a bucket with the `clippath` of its end tag, folded to
/// one of 64 buckets.
pub(in super::super) fn bucket(name: &str) -> usize {
    let mut h: u32 = 0x811C_9DC5;
    for &c in name.as_bytes() {
        h = (h ^ u32::from(c.to_ascii_lowercase())).wrapping_mul(0x0100_0193);
    }
    (h as usize) & 63
}
