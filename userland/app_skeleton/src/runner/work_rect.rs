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

//! A rect an app asks for, in thousandths of the work area, and the pixels it
//! comes to on a display (`rect_ask.rs`).

/// A rect in thousandths of the work area: no display needed to ask.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WorkRect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

/// A window rect in pixels: x, y, width, height.
pub type Rect = (u32, u32, u32, u32);

/// `r` in pixels within `work`, at least one pixel each way and never past it.
pub fn resolve(r: WorkRect, work: Rect) -> Rect {
    let part = |v: u16, of: u32| (u64::from(v) * u64::from(of) / 1000) as u32;
    let (w, h) = (part(r.w, work.2).max(1), part(r.h, work.3).max(1));
    let x = work.0 + part(r.x, work.2).min(work.2.saturating_sub(w));
    let y = work.1 + part(r.y, work.3).min(work.3.saturating_sub(h));
    (x, y, w.min(work.2.max(1)), h.min(work.3.max(1)))
}
