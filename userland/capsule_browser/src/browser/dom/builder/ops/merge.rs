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

use crate::browser::html::tokenizer::MAX_TAG_ATTRS;

use super::super::super::limits::{MAX_ATTRS, MAX_ATTR_BYTES, TRUNC_ATTRS};
use super::state::Builder;

impl Builder {
    /// Give `id` the attributes it does not already have, as a repeated html
    /// or body start tag does. Bounded like any tag's attributes, so a page
    /// of repeated tags cannot grow one element without limit.
    pub(in super::super) fn merge_attrs(&mut self, id: usize, attrs: Vec<(String, String)>) {
        for (name, value) in attrs {
            let have = &self.dom.nodes[id].attrs;
            if have.iter().any(|(n, _)| *n == name) {
                continue;
            }
            if have.len() >= MAX_TAG_ATTRS || !self.charge(name.len() + value.len()) {
                self.dom.truncated |= TRUNC_ATTRS;
                return;
            }
            self.dom.nodes[id].attrs.push((name, value));
        }
    }

    /// Count one attribute of `bytes` against the document's budget, or
    /// refuse it when the budget is spent.
    pub(in super::super) fn charge(&mut self, bytes: usize) -> bool {
        if self.attrs_used >= MAX_ATTRS || self.attr_bytes + bytes > MAX_ATTR_BYTES {
            self.dom.truncated |= TRUNC_ATTRS;
            return false;
        }
        self.attrs_used += 1;
        self.attr_bytes += bytes;
        true
    }
}
