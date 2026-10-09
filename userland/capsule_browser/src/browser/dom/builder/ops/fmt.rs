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

use super::kinds::FORMATTING;
use super::mode::Entry;
use super::state::{Builder, IN_FMT};

/// Entries the list of active formatting elements keeps (the specification
/// sets no limit); past this the oldest element entry is forgotten.
const MAX_FMT: usize = 128;

impl Builder {
    /// Push onto the list of active formatting elements, with the Noah's Ark
    /// clause: at most three entries per tag and attribute set after a marker.
    pub(in super::super) fn push_fmt(&mut self, id: usize) {
        self.fmt_mark(id);
        let start = self.fmt.iter().rposition(|e| *e == Entry::Marker).map_or(0, |i| i + 1);
        let kind = self.flags[id] >> 2;
        let mut same = 0;
        let mut earliest = None;
        for i in (start..self.fmt.len()).rev() {
            if let Entry::Elem(e) = self.fmt[i] {
                if self.flags[e] >> 2 == kind && self.same_element(e, id) {
                    same += 1;
                    earliest = Some(e);
                }
            }
        }
        if let (true, Some(e)) = (same >= 3, earliest) {
            self.remove_fmt(e);
        }
        if self.fmt.len() >= MAX_FMT {
            if let Some(Entry::Elem(e)) = self.fmt.iter().copied().find(|e| *e != Entry::Marker) {
                self.remove_fmt(e);
            }
        }
        self.fmt.push(Entry::Elem(id));
    }

    /// Mark `id` as in the list, with its formatting tag folded into the
    /// flags byte so list walks compare one byte before reading the node.
    pub(in super::super) fn fmt_mark(&mut self, id: usize) {
        let tag = self.dom.nodes[id].tag.as_str();
        let kind = FORMATTING.iter().position(|f| *f == tag).map_or(0, |k| k as u8 + 1);
        self.flags[id] = (self.flags[id] & !(0x3F << 2)) | IN_FMT | (kind << 2);
    }

    /// The last element named `subject` after the last marker in the list.
    pub(in super::super) fn fmt_last_named(&self, subject: &str) -> Option<usize> {
        let kind = FORMATTING.iter().position(|f| *f == subject).map_or(0, |k| k as u8 + 1);
        for e in self.fmt.iter().rev() {
            match *e {
                Entry::Marker => return None,
                Entry::Elem(id) if self.flags[id] >> 2 == kind && self.is(id, subject) => {
                    return Some(id)
                }
                Entry::Elem(_) => {}
            }
        }
        None
    }
}
