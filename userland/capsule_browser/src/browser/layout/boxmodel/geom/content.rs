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

use alloc::string::String;

use crate::browser::css::ObjectFit;

/* What a fragment paints inside its box: nothing, a run of text, or an image. */
pub enum Content {
    None,
    Text {
        text: String,
        color: u32,
        px: f32,
        bold: bool,
        mono: bool,
        underline: bool,
        font: u32,
        spacing: f32,
        /* font-style italic or oblique: drawn slanted. */
        italic: bool,
    },
    Image {
        src: String,
        alt: String,
        fit: ObjectFit,
    },
}
