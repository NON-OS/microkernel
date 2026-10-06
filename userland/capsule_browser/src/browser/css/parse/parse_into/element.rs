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

use crate::browser::css::pseudo_style::PseudoText as P;

/* Trailing pseudo-elements the cascade styles, and their Selector::element
 * codes (the codes the selector parser assigns them). The selector parser
 * reads ::before and ::after itself. */
const ELEMENTS: [(&str, u8); 9] = [
    ("::marker", P::MARKER),
    ("::placeholder", P::PLACEHOLDER),
    ("::-webkit-input-placeholder", P::PLACEHOLDER),
    ("::first-letter", P::FIRST_LETTER),
    (":first-letter", P::FIRST_LETTER),
    ("::first-line", P::FIRST_LINE),
    (":first-line", P::FIRST_LINE),
    ("::file-selector-button", P::FILE_BUTTON),
    ("::-webkit-file-upload-button", P::FILE_BUTTON),
];

/* A selector's host part and the code of the pseudo-element it ends
 * in, 0 when it ends in none of those. */
pub(super) fn element_of(t: &str) -> (&str, u8) {
    for (name, code) in ELEMENTS {
        let cut = t.len().saturating_sub(name.len());
        if t.is_char_boundary(cut) && t[cut..].eq_ignore_ascii_case(name) {
            return (&t[..cut], code);
        }
    }
    (t, 0)
}
