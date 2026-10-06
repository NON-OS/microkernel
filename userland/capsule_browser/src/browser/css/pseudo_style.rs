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

mod exists;
mod one;
mod pass;

use alloc::string::String;

use super::computed::Computed;
pub(super) use pass::{after, before};

/* One pseudo-element of an element, cascaded from the element's style
 * like a child's: a ::before or ::after box with its generated text
 * (possibly empty, which still makes a box), or the style of ::marker
 * (with its content text when one was given), ::placeholder,
 * ::first-letter, ::first-line or ::file-selector-button. */
pub struct PseudoText {
    pub kind: u8,
    pub text: Option<String>,
    pub style: Computed,
    pub bg_image: Option<String>,
}

impl PseudoText {
    /* The kinds, which are the Selector::element codes. */
    pub const BEFORE: u8 = 1;
    pub const AFTER: u8 = 2;
    pub const MARKER: u8 = 3;
    pub const PLACEHOLDER: u8 = 4;
    pub const FIRST_LETTER: u8 = 5;
    pub const FIRST_LINE: u8 = 6;
    pub const FILE_BUTTON: u8 = 7;
}
