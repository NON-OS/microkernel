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

use super::draw::draw_entry_at;
use crate::display::log_panel::buffer::{get_count, get_entry};
use crate::display::log_panel::types::{LogLevel, MAX_LOG_LINES};

/* The splash shows one line under its headline: the latest warning, error or
security note. Everything else (results, addresses, hashes, stage names) is in
the step list or on the serial console. */
fn latest_shown(total: usize) -> Option<usize> {
    let kept = total.min(MAX_LOG_LINES);
    (0..kept).map(|back| (total - 1 - back) % MAX_LOG_LINES).find(|&i| {
        get_entry(i).is_some_and(|e| e.len > 0 && shown(e.level))
    })
}

fn shown(level: LogLevel) -> bool {
    matches!(level, LogLevel::Warn | LogLevel::Error | LogLevel::Security)
}

pub fn redraw_all() {
    let count = get_count();
    if let Some(i) = latest_shown(count) {
        draw_entry_at(0, i);
    }
}

pub fn render_after_log(count: usize) {
    if count == 0 {
        return;
    }
    let i = (count - 1) % MAX_LOG_LINES;
    if get_entry(i).is_some_and(|e| shown(e.level)) {
        draw_entry_at(0, i);
    }
}
