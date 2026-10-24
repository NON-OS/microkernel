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
//! A touchpad's frame as the one sample the gesture engine reads: the first
//! finger down, and how many are down. Read from every slot, a pad whose
//! first slot lifts while another finger stays keeps that finger, where
//! the first slot alone would have read as every finger lifting.

use super::super::decode::TouchSample;
use super::frame::Frame;

pub fn primary(f: &Frame, max: (i32, i32)) -> TouchSample {
    let mut down = f.touches[..f.n].iter().filter(|t| t.tip);
    let first = down.next().copied();
    let t = first.unwrap_or_default();
    TouchSample {
        x: t.x,
        y: t.y,
        x_max: max.0,
        y_max: max.1,
        tip: first.is_some(),
        contacts: first.map_or(0, |_| 1 + down.count() as u32),
        contact_id: first.map(|t| t.id),
        button: f.button,
        confidence: first.is_none_or(|t| t.confidence),
    }
}
