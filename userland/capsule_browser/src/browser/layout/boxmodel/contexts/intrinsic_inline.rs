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

use super::super::inline_items::Lead;
use super::super::tree::BoxNode;
use super::inline_flow::Flow;
use super::inline_sink::Sink;
use super::inline_walk::walk;
use super::intrinsic::contribution;

/* (min-content, max-content) of an inline run, from the same walk line
 * layout makes: the same words, spaces and break chances. max-content is
 * the longest line between forced breaks; min-content the widest stretch
 * no line may break inside. */
pub(in super::super) fn inline_run(children: &[BoxNode], depth: u32) -> (i32, i32) {
    let mut m = Measure { line: 0, max: 0, seg: 0, min: 0 };
    walk(children, &mut Flow::new(), &mut m, depth + 1);
    m.hard_break();
    (m.min, m.max)
}

struct Measure {
    line: i32,
    max: i32,
    seg: i32,
    min: i32,
}

impl Measure {
    fn item(&mut self, lead: Lead, min_w: i32, max_w: i32) {
        self.line = self.line.saturating_add(lead.space).saturating_add(max_w);
        if lead.brk {
            self.min = self.min.max(self.seg);
            self.seg = min_w;
        } else {
            self.seg = self.seg.saturating_add(lead.space).saturating_add(min_w);
        }
    }
}

impl Sink for Measure {
    fn word(&mut self, _: &BoxNode, _: String, adv: i32, lead: Lead) {
        self.item(lead, adv, adv);
    }

    /* An atom with its margins, at its own min and max widths. */
    fn atom(&mut self, c: &BoxNode, lead: Lead, depth: u32) {
        let (a, b) = contribution(c, depth);
        self.item(lead, a, b);
    }

    fn edge(&mut self, _: &BoxNode, w: i32, _: u32, lead: Lead) {
        self.item(lead, w, w);
    }

    fn hard_break(&mut self) {
        (self.max, self.min) = (self.max.max(self.line), self.min.max(self.seg));
        (self.line, self.seg) = (0, 0);
    }
}
