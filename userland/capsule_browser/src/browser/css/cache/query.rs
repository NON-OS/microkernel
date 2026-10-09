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

use crate::browser::css::compute::Styled;
use crate::browser::css::rule::Rule;

use super::CssCache;

impl CssCache {
    /* The kept styles, once restyle has run. */
    pub fn styled(&self) -> Option<&Styled> {
        self.memo.as_ref().map(|m| &m.styled)
    }

    /* Whether a layout of the document with fingerprint `print` at
     * `viewport` would differ from the last one, recording this one. */
    pub fn relaid(&mut self, print: u64, viewport: (u32, u32)) -> bool {
        self.laid.replace((print, viewport)) != Some((print, viewport))
    }

    /* Which element states some selector tests: 1 :hover, 2 :focus (any
     * form), 4 :active. A change of any other state restyles nothing. */
    pub fn state_mask(&self) -> u8 {
        let f = self.author.flags;
        let states = [(Rule::HOVER, 1), (Rule::FOCUS, 2), (Rule::ACTIVE, 4)];
        states.iter().filter(|s| f & s.0 != 0).fold(0, |m, s| m | s.1)
    }

    pub(super) fn style_key(&self, print: (u64, u64)) -> u64 {
        if self.author.flags & Rule::VALUE_ATTR != 0 {
            print.0
        } else {
            print.1
        }
    }
}
