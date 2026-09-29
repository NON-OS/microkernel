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

//! The user-agent style sheet, after the rendering section of the HTML
//! standard as Chromium ships it: display roles, the flow and heading
//! defaults, text-level styles, tables and form controls. <noscript> is
//! not in it: whether its content renders is the cascade's policy
//! (noscript_visible).

mod blocks;
mod forms;
mod tables;
mod text;

use alloc::vec::Vec;

use super::parse::parse;
use super::rule::Rule;

pub fn ua_rules() -> Vec<Rule> {
    let mut rules = Vec::new();
    for sheet in [blocks::SHEET, text::SHEET, tables::SHEET, forms::SHEET] {
        rules.extend(parse(sheet));
    }
    rules
}

/* Whether <noscript> content renders. With scripting on, a browser hides
 * it; here it also renders when the page's scripts cannot stand in for
 * it: QuickJS is off (`js_on` false), the scripts are known to have
 * failed (`scripts_ok` Some(false); None is not known), or they left no
 * body text outside <noscript> (`text_outside` false), as on a
 * server-rendered forum whose content sits in <noscript> for crawlers. */
pub fn noscript_visible(js_on: bool, scripts_ok: Option<bool>, text_outside: bool) -> bool {
    !js_on || scripts_ok == Some(false) || !text_outside
}
