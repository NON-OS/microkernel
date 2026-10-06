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
use super::pseudo_element::legacy_element;

/* Scrollbar parts, which take the scrollbar pseudo-classes. */
const SCROLLBAR_PARTS: [&str; 7] = [
    "-webkit-scrollbar",
    "-webkit-scrollbar-button",
    "-webkit-scrollbar-thumb",
    "-webkit-scrollbar-track",
    "-webkit-scrollbar-track-piece",
    "-webkit-scrollbar-corner",
    "-webkit-resizer",
];

/* -webkit- names Blink knows as pseudo-classes, which it therefore rejects
 * after '::'; every other ::-webkit- name is a valid custom element. */
const WEBKIT_CLASSES: [&str; 6] = [
    "-webkit-any-link",
    "-webkit-autofill",
    "-webkit-drag",
    "-webkit-full-page-media",
    "-webkit-full-screen",
    "-webkit-full-screen-ancestor",
];

/* A pseudo-element name without an argument, lower-cased. */
pub(super) fn element_name(name: &str) -> Option<u8> {
    Some(match name {
        "marker" => MARKER,
        "placeholder" => PLACEHOLDER,
        "selection" => SELECTION,
        "backdrop" => BACKDROP,
        "file-selector-button" | "-webkit-file-upload-button" => FILE_BUTTON,
        "cue" => CUE,
        "view-transition" => TRANSITION,
        "target-text"
        | "spelling-error"
        | "grammar-error"
        | "details-content"
        | "picker-icon"
        | "checkmark"
        | "scroll-marker"
        | "scroll-marker-group"
        | "column" => OTHER,
        n if SCROLLBAR_PARTS.contains(&n) => SCROLLBAR,
        n if n.starts_with("-webkit-") && !WEBKIT_CLASSES.contains(&n) => CUSTOM,
        n => return legacy_element(n),
    })
}
