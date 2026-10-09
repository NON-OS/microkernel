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

//! OSC 8 hyperlinks: text a program marked as pointing somewhere. The host
//! shows where before following one; the target is never followed here.

use super::dispatch::clean_text;
use crate::limits::{MAX_LINK, MAX_LINKS};
use crate::term::state::Term;

impl Term {
    /// `8;params;uri` starts a link, `8;;` ends it.
    pub(in crate::term) fn osc_link(&mut self, rest: &[u8]) {
        let Some(i) = rest.iter().position(|&b| b == b';') else { return };
        let uri = &rest[i + 1..];
        let link = if uri.is_empty() || uri.len() > MAX_LINK { 0 } else { self.intern_link(uri) };
        self.scr().cur.pen.link = link;
    }

    fn intern_link(&mut self, uri: &[u8]) -> u16 {
        let uri = clean_text(uri, MAX_LINK);
        if let Some(i) = self.links.iter().position(|l| *l == uri) {
            return (i + 1) as u16;
        }
        if self.links.len() >= MAX_LINKS {
            return 0;
        }
        self.links.push(uri);
        self.links.len() as u16
    }

    /// The target a cell's link names.
    pub fn link_target(&self, link: u16) -> Option<&str> {
        let i = (link as usize).checked_sub(1)?;
        self.links.get(i).map(|s| s.as_str())
    }
}
