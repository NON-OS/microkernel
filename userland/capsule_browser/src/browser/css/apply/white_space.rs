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

use crate::browser::css::computed::{Computed, WhiteSpace};

/* white-space: how inline layout treats spaces, tabs and newlines. The
 * shorthand's white-space-collapse and text-wrap longhands map onto the
 * same modes. */
pub(super) fn apply_white_space(c: &mut Computed, name: &str, value: &str) -> bool {
    let ws = match (name, value.trim()) {
        ("white-space", "normal") => WhiteSpace::Normal,
        ("white-space", "nowrap") | ("text-wrap" | "text-wrap-mode", "nowrap") => {
            WhiteSpace::Nowrap
        }
        ("white-space", "pre") => WhiteSpace::Pre,
        ("white-space", "pre-wrap" | "break-spaces") => WhiteSpace::PreWrap,
        ("white-space", "pre-line") => WhiteSpace::PreLine,
        ("white-space-collapse", "preserve" | "break-spaces") => match c.white_space {
            WhiteSpace::Nowrap | WhiteSpace::Pre => WhiteSpace::Pre,
            _ => WhiteSpace::PreWrap,
        },
        ("white-space-collapse", "preserve-breaks") => WhiteSpace::PreLine,
        /* wrap, balance and pretty all wrap; only the mode is modelled. */
        ("text-wrap" | "text-wrap-mode", _) if c.white_space == WhiteSpace::Nowrap => {
            WhiteSpace::Normal
        }
        ("white-space-collapse", "collapse") => match c.white_space {
            WhiteSpace::Nowrap | WhiteSpace::Pre => WhiteSpace::Nowrap,
            _ => WhiteSpace::Normal,
        },
        ("white-space" | "white-space-collapse" | "text-wrap" | "text-wrap-mode", _) => {
            return true
        }
        _ => return false,
    };
    c.white_space = ws;
    true
}
