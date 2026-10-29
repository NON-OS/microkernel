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

//! One message bubble laid out to a width: its lines, author, time and status.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::measure_ttf;

use super::fit::wrap;
use super::metrics::{Metrics, BODY_PX, LINE, PAD, SMALL_LINE, SMALL_PX};
use crate::proto::item::{clock, Item};
use crate::proto::status::Status;

pub struct Bubble {
    pub lines: Vec<String>,
    pub author: String,
    pub meta: String,
    pub sent: bool,
    pub failed: bool,
    pub w: u32,
    pub h: u32,
}

pub(super) fn width(text: &str, px: f32) -> u32 {
    measure_ttf(text, px).max(0) as u32
}

pub fn layout(item: &Item, max_w: u32, m: Metrics) -> Bubble {
    let pad = m.px(PAD);
    let lines = wrap(&item.text, BODY_PX, max_w.saturating_sub(2 * pad).max(1));
    let meta = match item.status.label() {
        "" => String::from(clock(&item.ts)),
        said => format!("{}  {said}", clock(&item.ts)),
    };
    let author = if item.sent { String::new() } else { item.author.clone() };
    let widest = lines.iter().map(|l| width(l, BODY_PX)).max().unwrap_or(0);
    let inner = widest.max(width(&meta, SMALL_PX)).max(width(&author, SMALL_PX));
    let head = if author.is_empty() { 0 } else { m.px(SMALL_LINE) };
    let h = 2 * pad + head + lines.len() as u32 * m.px(LINE) + m.px(SMALL_LINE);
    let failed = item.status == Status::Failed;
    Bubble { lines, author, meta, sent: item.sent, failed, w: (inner + 2 * pad).min(max_w), h }
}
