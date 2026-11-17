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

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Rect, Toast, Tone};

use crate::shell_layer::layer;
use crate::shell_toast_view::{bounds, draw, hit, stack};
use crate::store_state::NotifyLevel;

const LAYER: Rect = Rect { x: 0, y: 0, w: 960, h: 540 };
const LONG: &str = "The capsule store could not be read, so no Linux programs this boot";

fn painted(toasts: &[Toast], boxes: &[Rect]) -> Vec<u32> {
    let (w, h) = (LAYER.w, LAYER.h);
    let mut words = vec![roles().bg.argb(); (w * h) as usize];
    draw(&mut unsafe { layer(words.as_mut_ptr() as u64, w * 4, w, h) }, toasts, boxes);
    words
}

fn extent(words: &[u32]) -> Rect {
    let bg = roles().bg.argb();
    let on: Vec<_> = (0..words.len()).filter(|&i| words[i] != bg).map(|i| i as u32).collect();
    let (xs, ys) = (on.iter().map(|i| i % LAYER.w), on.iter().map(|i| i / LAYER.w));
    let (x, y) = (xs.clone().min().unwrap(), ys.clone().min().unwrap());
    Rect { x, y, w: xs.max().unwrap() + 1 - x, h: ys.max().unwrap() + 1 - y }
}

#[test]
fn each_notify_level_is_a_tone() {
    let levels = [NotifyLevel::Info, NotifyLevel::Warn, NotifyLevel::Error];
    assert_eq!(levels.map(NotifyLevel::tone), [Tone::Accent, Tone::Warn, Tone::Danger]);
}

pub(crate) fn toasts_stack_in_their_boxes(tag: &str) {
    let levels = [NotifyLevel::Info, NotifyLevel::Warn, NotifyLevel::Error];
    let said = ["Terminal opened", LONG, "Calculator did not start"];
    let toasts: Vec<_> = (0..3).map(|i| Toast::new(said[i]).tone(levels[i].tone())).collect();
    let (right, foot) = (LAYER.w - 12, LAYER.h - 40);
    let boxes = stack(&toasts, right, foot, LAYER.w - 24);
    assert_eq!(boxes.last().map(|b| b.y + b.h), Some(foot), "{tag}: the last sits on the foot");
    let apart = boxes.windows(2).all(|p| p[0].y + p[0].h < p[1].y);
    assert!(apart, "{tag}: toasts overlap or touch: {boxes:?}");
    assert!(boxes[1].h > boxes[0].h, "{tag}: the long toast takes two lines");
    for (i, (toast, at)) in toasts.iter().zip(&boxes).enumerate() {
        assert_eq!(at.x + at.w, right, "{tag}: toast {i} is not right-aligned");
        let alone = painted(core::slice::from_ref(toast), core::slice::from_ref(at));
        assert_eq!(extent(&alone), *at, "{tag}: toast {i} is not drawn in its box");
        let bar = levels[i].tone().colour(&roles()).argb();
        let mut x = (0..LAYER.w * LAYER.h).filter(|&p| alone[p as usize] == bar).peekable();
        assert!(x.peek().is_some(), "{tag}: toast {i} is not in its tone");
        assert!(x.all(|p| p % LAYER.w < at.x + at.w / 2), "{tag}: toast {i} tone off its bar");
    }
    let whole = bounds(&boxes).unwrap();
    assert_eq!(extent(&painted(&toasts, &boxes)), whole, "{tag}: the stack is not its bounds");
    let (x0, y0) = (whole.x as i32, whole.y as i32);
    let (x1, y1) = (x0 + whole.w as i32, y0 + whole.h as i32);
    assert!(hit(&boxes, x0, y0) && hit(&boxes, x1 - 1, y1 - 1), "{tag}: a corner is not hit");
    assert!(!hit(&boxes, x0 - 1, y0) && !hit(&boxes, x1, y1 - 1), "{tag}: hit left of or past it");
    assert!(!hit(&boxes, x0, y0 - 1) && !hit(&boxes, x0, y1), "{tag}: hit above or below it");
    assert!(!hit(&[], x0, y0), "{tag}: no toasts, no hit");
}
