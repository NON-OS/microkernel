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

//! Text fitted to a width: cut with an ellipsis, or wrapped into lines.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::measure_ttf;

fn width(text: &str, px: f32) -> u32 {
    measure_ttf(text, px).max(0) as u32
}

/// `text` on one line within `max` pixels, its tail cut to three dots.
pub fn ellipsize(text: &str, px: f32, max: u32) -> String {
    let line = text.lines().next().unwrap_or("");
    if width(line, px) <= max {
        return String::from(line);
    }
    let mut out = String::from(line);
    while !out.is_empty() {
        out.pop();
        if width(&out, px) + width("...", px) <= max {
            break;
        }
    }
    out.push_str("...");
    out
}

/// `text` wrapped at spaces into lines no wider than `max`; a word wider than
/// a line is broken where it reaches the edge.
pub fn wrap(text: &str, px: f32, max: u32) -> Vec<String> {
    let mut lines = Vec::new();
    for para in text.split('\n') {
        let mut line = String::new();
        for word in para.split(' ') {
            let sep = if line.is_empty() { "" } else { " " };
            let joined = [line.as_str(), sep, word].concat();
            if width(&joined, px) <= max || line.is_empty() && word.is_empty() {
                line = joined;
                continue;
            }
            if !line.is_empty() {
                lines.push(core::mem::take(&mut line));
            }
            for ch in word.chars() {
                line.push(ch);
                if width(&line, px) > max && line.chars().count() > 1 {
                    let last = line.pop().unwrap_or(ch);
                    lines.push(core::mem::replace(&mut line, String::from(last)));
                }
            }
        }
        lines.push(line);
    }
    lines
}
