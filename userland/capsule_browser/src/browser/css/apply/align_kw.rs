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

use crate::browser::css::computed::{Align, Justify};

/* The keyword after an optional safe or unsafe overflow modifier. */
fn bare(v: &str) -> &str {
    let v = v.trim();
    for m in ["unsafe ", "safe "] {
        if let Some(rest) = v.strip_prefix(m) {
            return rest.trim();
        }
    }
    v
}

/* A self or items alignment keyword: Some(None) for normal and auto, which
 * leave the choice to the layout mode; None when the value is not one. The
 * baselines sit at the start, where the first line's baseline is. */
pub(super) fn self_kw(v: &str) -> Option<Option<Align>> {
    Some(match bare(v) {
        "auto" | "normal" | "legacy" => None,
        "stretch" => Some(Align::Stretch),
        "center" | "legacy center" => Some(Align::Center),
        "start" | "flex-start" | "self-start" | "left" | "legacy left" | "baseline"
        | "first baseline" => Some(Align::Start),
        "end" | "flex-end" | "self-end" | "right" | "legacy right" | "last baseline" => {
            Some(Align::End)
        }
        _ => return None,
    })
}

/* A justify-content keyword. normal and stretch pack flex items at the
 * start, which is what they do when no item can stretch. */
pub(super) fn justify_kw(v: &str) -> Option<Justify> {
    Some(match bare(v) {
        "flex-start" | "start" | "left" | "normal" | "stretch" => Justify::Start,
        "center" => Justify::Center,
        "flex-end" | "end" | "right" => Justify::End,
        "space-between" => Justify::Between,
        "space-around" => Justify::Around,
        "space-evenly" => Justify::Evenly,
        _ => return None,
    })
}
