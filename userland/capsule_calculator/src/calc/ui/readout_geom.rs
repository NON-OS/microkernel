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

use super::metrics::{metrics, px};

/// How much nearer the readout's edge its text sits than the pane gutter, in
/// logical pixels.
const TUCK: i32 = 4;

pub fn origin() -> (i32, i32) {
    let m = metrics();
    (m.rail_w + m.pane_pad, m.pane_pad)
}

pub fn size(win_w: i32) -> (i32, i32) {
    let m = metrics();
    (win_w - m.rail_w - m.pane_pad * 2, m.readout_h)
}

pub fn inset() -> i32 {
    metrics().pane_pad - px(TUCK)
}
