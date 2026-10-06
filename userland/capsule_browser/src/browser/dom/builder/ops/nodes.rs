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
use super::state::Builder;

impl Builder {
    /// The current node: the bottommost open element, or the document when
    /// nothing is open yet.
    pub(in super::super) fn cur(&self) -> usize {
        self.open.last().copied().unwrap_or(0)
    }

    /// The adjusted current node: the fragment context while only the root
    /// is open, otherwise the current node.
    pub(in super::super) fn adjusted(&self) -> usize {
        match self.context {
            Some(ctx) if self.open.len() == 1 => ctx,
            _ => self.cur(),
        }
    }

    /// Whether `id` is an HTML element named `name`.
    pub(in super::super) fn is(&self, id: usize, name: &str) -> bool {
        let n = &self.dom.nodes[id];
        n.ns == Ns::Html && n.tag == name
    }

    /// Whether `id` is an HTML element with one of these names.
    pub(in super::super) fn is_any(&self, id: usize, names: &[&str]) -> bool {
        let n = &self.dom.nodes[id];
        n.ns == Ns::Html && names.contains(&n.tag.as_str())
    }

    pub(in super::super) fn cur_is(&self, name: &str) -> bool {
        self.is(self.cur(), name)
    }

    pub(in super::super) fn cur_is_any(&self, names: &[&str]) -> bool {
        self.is_any(self.cur(), names)
    }

    /// Whether a template element is on the stack of open elements.
    pub(in super::super) fn template_open(&self) -> bool {
        self.maybe_open("template") && self.open.iter().any(|&id| self.is(id, "template"))
    }

    /// Whether the parser is parsing template contents: a template is open,
    /// or the fragment being parsed is a template's.
    pub(in super::super) fn parsing_template(&self) -> bool {
        self.template_open() || self.context_is("template")
    }

    /// Whether this is a fragment parsed for an HTML element named `name`.
    pub(in super::super) fn context_is(&self, name: &str) -> bool {
        self.context.is_some_and(|c| self.is(c, name))
    }
}
