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

use super::super::super::node::Ns;
use super::meta::RESETS;
use super::mode::Mode;
use super::state::Builder;

impl Builder {
    /// Reset the insertion mode appropriately (13.2.4.1), from the elements
    /// still open, and in a fragment from its context.
    pub(in super::super) fn reset_mode(&mut self) {
        for i in (0..self.open.len()).rev() {
            let last = i == 0;
            if !last && self.open_meta[i] & RESETS == 0 {
                continue;
            }
            let node = match self.context {
                Some(ctx) if last => ctx,
                _ => self.open[i],
            };
            let n = &self.dom.nodes[node];
            let tag = if n.ns == Ns::Html { n.tag.as_str() } else { "" };
            let mode = match tag {
                "td" | "th" if !last => Mode::InCell,
                "tr" => Mode::InRow,
                "tbody" | "thead" | "tfoot" => Mode::InTableBody,
                "caption" => Mode::InCaption,
                "colgroup" => Mode::InColumnGroup,
                "table" => Mode::InTable,
                /*
                 * A template left open without its mode (the stack cap
                 * closed it) reads as body content.
                 */
                "template" => self.tmpl.last().copied().unwrap_or(Mode::InBody),
                "head" if !last => Mode::InHead,
                "body" => Mode::InBody,
                "frameset" => Mode::InFrameset,
                "html" if self.head.is_none() => Mode::BeforeHead,
                "html" => Mode::AfterHead,
                _ if last => Mode::InBody,
                _ => continue,
            };
            self.mode = mode;
            return;
        }
        self.mode = Mode::InBody;
    }
}
