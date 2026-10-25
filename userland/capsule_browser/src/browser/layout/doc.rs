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

use crate::browser::html::flow::Flow;

pub const LINE_H: u32 = 20;

pub struct Span {
    pub x: u32,
    pub w: u32,
    pub text: String,
    pub color: u32,
    pub bg: u32,
    pub href: Option<String>,
    pub scale: u32,
    pub bold: bool,
}

pub struct RenderLine {
    pub y: u32,
    pub height: u32,
    pub spans: Vec<Span>,
}

pub struct RenderDocument {
    pub lines: Vec<RenderLine>,
    pub content_h: u32,
    /// The width it was laid out at, and what from, so it can be laid out
    /// again at the width of the window it is shown in (app/paint.rs).
    pub width: u32,
    pub source: Vec<Flow>,
}

impl RenderDocument {
    /// Laid out again at `width`, from the flows it was built from.
    pub fn reflow(&mut self, width: u32, advance: u32) {
        let source = core::mem::take(&mut self.source);
        *self = super::build::build(&source, width, advance);
    }

    pub fn link_at(&self, x: i32, y: i32) -> Option<&str> {
        for line in &self.lines {
            let sy = line.y as i32;
            for s in &line.spans {
                if let Some(href) = s.href.as_deref() {
                    let sx = s.x as i32;
                    if x >= sx && x < sx + s.w as i32 && y >= sy && y < sy + line.height as i32 {
                        return Some(href);
                    }
                }
            }
        }
        None
    }
}
