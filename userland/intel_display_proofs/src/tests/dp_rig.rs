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

//! A DisplayPort monitor firmware lit on DDI B through transcoder and pipe
//! B, on DPLL 1, taken over as the capsule's bring-up takes it: surfaces
//! holding firmware's picture (0xAA, 0xBB), the monitor's EDID read over
//! AUX, mode setting found. The GGTT's size is the test's.

use std::cell::RefCell;
use std::rc::Rc;

use super::dp_board::dp_board;
use super::wire_rig::{taken, Rig};
use crate::display::{Intel, Modeset, Monitor};
use crate::engine::gen::GEN9;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::scanout::Scanout;
use crate::engine::takeover::read_output;
use crate::model::firmware::Axis;
use crate::model::sink::Sink;
use crate::model::{Gen9, ModelFrames, Polls, Shared, Ticks};

pub type Model = Rc<RefCell<Gen9>>;

pub fn dp_rig(timing: (Axis, Axis), edid: Vec<u8>, total: u32) -> (Rig, Model, Sink) {
    let (mut m, s, a, b) = dp_board(timing, edid, total);
    let pages = a.len() as u64;
    let out = read_output(&mut m, &GEN9, Pipe::B).unwrap();
    let scan = Scanout::start(&mut m, &out, total, [&a, &b], &Polls::new(50)).unwrap();
    let model = Rc::new(RefCell::new(m));
    let mut shared = Shared(model.clone());
    let clock = Ticks::new();
    let monitor = Monitor::of(&mut shared, &clock, &GEN9, Transcoder::B);
    let modeset = Modeset::find(&mut shared, &clock, &GEN9, &out, &monitor, (0, total));
    let (iova, len) = ([a[0], b[0]], pages as usize * 4096);
    let next = Some(0x4000_0000);
    let frames = ModelFrames { model: shared.clone(), iova, len, fences: 0, was: None, next };
    let head = taken(out, scan, frames, (monitor.panel(), modeset), (0, total));
    let heads = [Some(head), None, None, None];
    let d = Intel::new(shared, clock, &GEN9, heads);
    (d, model, s)
}
