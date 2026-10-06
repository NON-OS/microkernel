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

/* Selector::element codes. The cascade styles 1 and 2 as generated
 * content; everything from 3 up is parsed so the rule stays valid, and the
 * cascade passes over it. */
pub(super) const BEFORE: u8 = 1;
pub(super) const AFTER: u8 = 2;
pub(super) const MARKER: u8 = 3;
pub(super) const PLACEHOLDER: u8 = 4;
pub(super) const FIRST_LETTER: u8 = 5;
pub(super) const FIRST_LINE: u8 = 6;
pub(super) const FILE_BUTTON: u8 = 7;
pub(super) const SELECTION: u8 = 8;
pub(super) const BACKDROP: u8 = 9;
/* A pseudo-element nested in or following another, and the rest. */
pub(super) const OTHER: u8 = 10;
/* ::-webkit-scrollbar and its parts. */
pub(super) const SCROLLBAR: u8 = 11;
/* Any other ::-webkit- name, which Blink accepts as a custom element. */
pub(super) const CUSTOM: u8 = 12;
pub(super) const PART: u8 = 13;
pub(super) const SLOTTED: u8 = 14;
pub(super) const CUE: u8 = 15;
pub(super) const TRANSITION: u8 = 16;
