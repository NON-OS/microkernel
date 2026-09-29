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

mod apply;
mod plan;
mod run;

use alloc::string::String;

use crate::browser::css::computed::Computed;
use crate::browser::css::grid_spec::GridSpec;
use crate::browser::css::vars::At;

/* One box being styled: its computed style and the values that live
 * beside it, with var() resolving through `vars`. */
pub(in crate::browser::css) struct Styling<'a> {
    pub c: Computed,
    pub bg: Option<String>,
    pub grid: Option<GridSpec>,
    /* The winning counter-reset, counter-increment and counter-set. */
    pub counters: [Option<String>; 3],
    /* The winning content value, for a pseudo-element. */
    pub content: Option<String>,
    pub(super) parent_fs: u32,
    pub(super) vars: At<'a>,
}

impl<'a> Styling<'a> {
    pub fn new(parent: &Computed, vars: At<'a>) -> Styling<'a> {
        let (c, parent_fs) = (Computed::inherit_from(parent), parent.font_size_px);
        Styling {
            c,
            bg: None,
            grid: None,
            counters: [None, None, None],
            content: None,
            parent_fs,
            vars,
        }
    }
}
