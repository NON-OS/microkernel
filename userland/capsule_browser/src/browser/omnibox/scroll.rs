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

/* A request to move the page. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAct {
    /* Arrow keys: lines down (negative: up). */
    Line(i32),
    /* Page Up/Down and Space: viewports down (negative: up). */
    Page(i32),
    Home,
    End,
    /* Wheel notches as the input router reports them: positive is up. */
    Wheel(i32),
    /* An absolute offset, such as an anchor's position. */
    To(i64),
}

pub const LINE_PX: i64 = 40;
pub const WHEEL_PX: i64 = 60;

/* One page keeps a tenth of the viewport in view as context, as other
 * browsers do, and never steps less than a line. */
pub fn page_step(view_h: u32) -> i64 {
    let v = view_h as i64;
    (v - v / 10).max(LINE_PX)
}

/* The farthest the page scrolls: its bottom meets the viewport bottom. */
pub fn max_scroll(content_h: u32, view_h: u32) -> u32 {
    content_h.saturating_sub(view_h)
}

/* The scroll offset after `act`, from `cur`, for a page `content_h` tall in
 * a viewport `view_h` tall. The sums run in i64 so no input overflows, and
 * the result is clamped to the scrollable range. */
pub fn scroll_to(cur: u32, content_h: u32, view_h: u32, act: ScrollAct) -> u32 {
    let max = max_scroll(content_h, view_h) as i64;
    let cur = cur as i64;
    let next = match act {
        ScrollAct::Line(n) => cur + n as i64 * LINE_PX,
        ScrollAct::Page(n) => cur + n as i64 * page_step(view_h),
        ScrollAct::Home => 0,
        ScrollAct::End => max,
        ScrollAct::Wheel(d) => cur - d as i64 * WHEEL_PX,
        ScrollAct::To(y) => y,
    };
    next.clamp(0, max) as u32
}
