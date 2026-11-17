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

//! The inspector's label/value lines. Identity and scheduling here, the
//! kernel's counters in `insp_fields_more.rs`.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Ink, Rect, Row as Line};

use crate::pm::format::{mem_human, percent, state_label, u32_decimal, uptime_human};
use crate::pm::format_labels::priority_label;
use crate::pm::format_mem::share_1dp;
use crate::pm::state::Row;

use super::metrics::metrics;
use super::text;
use super::tint::state_ink;

/// One label/value line as a shared row across the pane, whose padding puts
/// the label at `x`, the value at the right edge in `ink`. Returns the y the
/// next line starts at.
pub fn field(fb: &mut PaintBuffer, x: u32, y: u32, label: &[u8], value: &[u8], ink: Ink) -> u32 {
    let m = metrics();
    let at = Rect { x: x.saturating_sub(m.inspector_pad), y, w: m.inspector_w, h: m.insp_field_h };
    Line::new(text::valid(label)).value(text::valid(value)).value_ink(ink).draw(fb, at);
    y + m.insp_field_h
}

/// Who and how it is scheduled, then what it holds; `ram_kb` is installed
/// memory, so the share is of the machine and not of the table.
pub fn block(fb: &mut PaintBuffer, x: u32, y: u32, row: &Row, ram_kb: u64) -> u32 {
    let mut buf = [0u8; 24];
    let n = u32_decimal(row.pid, &mut buf);
    let mut y = field(fb, x, y, b"PID", &buf[..n], Ink::Text);
    let n = u32_decimal(row.ppid, &mut buf);
    y = field(fb, x, y, b"Parent", &buf[..n], Ink::Text2);
    y = field(fb, x, y, b"State", state_label(row.state), state_ink(row.state));
    y = field(fb, x, y, b"Priority", priority_label(row.priority), Ink::Text2);
    let n = uptime_human(row.uptime_ms / 1000, &mut buf);
    y = field(fb, x, y, b"Uptime", &buf[..n], Ink::Text2);
    let n = percent(row.cpu_pct, &mut buf);
    y = field(fb, x, y, b"CPU", &buf[..n], Ink::Text2);
    let n = mem_human(row.mem_kb, &mut buf);
    y = field(fb, x, y, b"Resident", &buf[..n], Ink::Text2);
    let n = share_1dp(row.mem_kb, ram_kb, &mut buf);
    y = field(fb, x, y, b"Share of RAM", &buf[..n], Ink::Text2);
    super::insp_fields_more::block(fb, x, y, row)
}
