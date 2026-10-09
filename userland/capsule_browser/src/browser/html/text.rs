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
use alloc::vec::Vec;

use crate::browser::html::flow::{Flow, Style};
use crate::browser::html::input::decode;

/// A text/plain body as lines. Decoded the way a page is, so one byte that
/// is not UTF-8 shows as U+FFFD instead of blanking the whole file.
pub fn parse_text(body: &[u8]) -> Vec<Flow> {
    let text = decode(body);
    let mut out = Vec::new();
    for line in text.lines().take(512) {
        out.push(Flow::Text(String::from(line), Style::default()));
        out.push(Flow::Break);
    }
    out
}
