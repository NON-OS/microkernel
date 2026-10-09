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

use super::element_code::*;

const USER: [&str; 5] = ["hover", "active", "focus", "focus-visible", "focus-within"];
const SCROLLBAR_CLASSES: [&str; 13] = [
    "horizontal",
    "vertical",
    "decrement",
    "increment",
    "start",
    "end",
    "double-button",
    "single-button",
    "no-button",
    "corner-present",
    "window-inactive",
    "hover",
    "active",
];

/* Whether Blink accepts `name` (a pseudo-element when `elem`, functional
 * when `func`) right after the pseudo-element coded `code`. */
pub(super) fn allowed(code: u8, elem: bool, name: &str, func: bool) -> bool {
    match (elem, func) {
        (true, false) => match code {
            BEFORE | AFTER => name == "marker",
            PART | SLOTTED => name == "before" || name == "after",
            _ => false,
        },
        (true, true) => false,
        (false, true) => match name {
            "is" | "where" => true,
            "not" => matches!(code, CUSTOM | FILE_BUTTON | PART | SCROLLBAR | CUE),
            "state" => code == PART,
            _ => false,
        },
        (false, false) => match code {
            CUSTOM | FILE_BUTTON | PART | CUE => USER.contains(&name),
            SCROLLBAR => SCROLLBAR_CLASSES.contains(&name),
            SELECTION => name == "window-inactive",
            TRANSITION => name == "only-child",
            _ => false,
        },
    }
}
