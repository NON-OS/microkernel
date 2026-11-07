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

use alloc::vec::Vec;

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Rect, Surface, Toast};

pub fn stack(toasts: &[Toast], right: u32, foot: u32, room: u32) -> Vec<Rect> {
    let gap = scale().px(u32::from(SPACE.xs));
    let mut top = foot.saturating_add(gap);
    let mut boxes: Vec<Rect> = toasts
        .iter()
        .rev()
        .map(|t| {
            let (w, h) = t.size();
            let w = w.min(room);
            top = top.saturating_sub(gap).saturating_sub(h);
            Rect { x: right.saturating_sub(w), y: top, w, h }
        })
        .collect();
    boxes.reverse();
    boxes
}

pub fn bounds(boxes: &[Rect]) -> Option<Rect> {
    let x = boxes.iter().map(|b| b.x).min()?;
    let y = boxes.iter().map(|b| b.y).min()?;
    let right = boxes.iter().map(|b| b.x + b.w).max()?;
    let foot = boxes.iter().map(|b| b.y + b.h).max()?;
    Some(Rect { x, y, w: right - x, h: foot - y })
}

pub fn hit(boxes: &[Rect], x: i32, y: i32) -> bool {
    bounds(boxes).is_some_and(|b| contains(b, x, y))
}

pub fn draw(fb: &mut impl Surface, toasts: &[Toast], boxes: &[Rect]) {
    for (toast, at) in toasts.iter().zip(boxes) {
        toast.draw(fb, *at);
    }
}
