// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::boot::layout::splash;
use crate::display::ink::{metrics, Style};

pub const MAX_LOG_LINES: usize = 256;
pub const LOG_LINE_LEN: usize = 120;

/// The splash shows the log's latest line under its headline; the whole log
/// goes to the serial console and the proofs panel shows the evidence.
pub fn line_height() -> u32 {
    metrics(Style::Mono).line
}

pub fn get_log_area() -> (u32, u32) {
    let s = splash();
    (s.col_x, s.detail_y)
}

pub fn max_visible_lines() -> usize {
    1
}

pub fn line_clear_width() -> u32 {
    splash().col_w
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Error,
    Security,
}
