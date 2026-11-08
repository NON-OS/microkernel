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
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Pointer, Rect, Scrolled};

use super::rail_column::Column;
use super::rail_lists::{draw_lists, Entry};
use super::rail_scroll::{telemetry_top, RailFit};
use super::rail_spot::base_name;
use super::rail_telemetry;
use super::rail_text::pad;
use super::tab_label::session_name;
use crate::rail::Rail;
use crate::term::prefs::types::Project;
use crate::term::state::State;

/// The one rail, painted as a single scrolled column on the window's chrome:
/// the sessions this window owns, the paths the user has pinned, then the
/// machine telemetry. Everything is drawn into a sub-buffer covering the
/// rail, so a band scrolled past either edge is cut instead of bleeding into
/// the rest of the frame.
pub fn draw(
    fb: &mut PaintBuffer,
    c: &Column,
    ptr: &Pointer,
    (tabs, active): (&[State], usize),
    projects: &[Project],
    rail: &Rail,
    telemetry: bool,
) {
    let r = c.rect;
    if r.w == 0 || r.h == 0 {
        return;
    }
    let mut sub = fb.sub(r.x, r.y, r.w, r.h);
    let fb = &mut sub;
    let ink = roles();
    draw::fill(fb, Rect { x: 0, y: 0, w: r.w, h: r.h }, ink.surface);
    let names: Vec<String> = (0..tabs.len()).map(name).collect();
    let busy = |i: usize| tabs[i].fg_running;
    let path = |i: usize| core::str::from_utf8(tabs[i].cwd.as_bytes()).unwrap_or("");
    let sessions: Vec<Entry> = names
        .iter()
        .enumerate()
        .map(|(i, n)| Entry { name: n, path: path(i), busy: busy(i) })
        .collect();
    let pinned: Vec<Entry> = projects
        .iter()
        .map(|p| Entry { name: base_name(p.as_str()), path: p.as_str(), busy: false })
        .collect();
    draw_lists(&mut Scrolled::new(fb, c.off), c, ptr, &sessions, &pinned, active);
    if telemetry {
        let fit = RailFit { sessions: c.sessions, projects: c.projects, telemetry };
        let w = r.w.saturating_sub(pad() * 2);
        rail_telemetry::draw(fb, c.off, pad(), telemetry_top(fit), w, rail);
    }
    let hair = scale().hair();
    draw::blend(fb, Rect { x: r.w.saturating_sub(hair), y: 0, w: hair, h: r.h }, ink.border);
}

fn name(i: usize) -> String {
    let mut buf = [0u8; 32];
    let n = session_name(i, &mut buf);
    String::from(core::str::from_utf8(&buf[..n]).unwrap_or(""))
}
