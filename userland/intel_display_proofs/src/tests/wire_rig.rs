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

//! A taken-over output on the model, as the capsule's bring-up leaves it:
//! the scanout started on two surfaces holding firmware's picture (here
//! 0xAA and 0xBB, so a test can tell which one shows), behind the wire.

use std::cell::RefCell;
use std::rc::Rc;

use super::scanout_tests::{setup, TOTAL};
use crate::display::{Head, Intel, Modeset, Plug};
use crate::engine::gen::GEN9;
use crate::engine::pipe::Pipe;
use crate::engine::scanout::Scanout;
use crate::engine::takeover::{read_output, Output};
use crate::model::{Gen9, ModelFrames, Polls, Shared, Ticks};
use nonos_gfx_serve::Panel;
use nonos_gfx_wire::body::state::SCANNING_OUT;

pub type Rig = Intel<Shared, ModelFrames, Ticks>;

pub fn rig() -> (Rig, Rc<RefCell<Gen9>>) {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, &GEN9, Pipe::A).unwrap();
    let scan = Scanout::start(&mut m, &out, TOTAL, [&a, &b], &Polls::new(50)).unwrap();
    let model = Rc::new(RefCell::new(m));
    let shared = Shared(model.clone());
    let len = scan.pages as usize * 4096;
    let (iova, next) = ([a[0], b[0]], Some(0x4000_0000));
    let frames = ModelFrames { model: shared.clone(), iova, len, fences: 0, was: None, next };
    let heads =
        [Some(taken(out, scan, frames, (Panel::default(), None), (0, TOTAL))), None, None, None];
    (Intel::new(shared, Ticks::new(), &GEN9, heads), model)
}

/// A head as the capsule's takeover leaves one: on its first surface,
/// scanning out, its monitor attached, no cursor yet.
pub fn taken(
    out: Output,
    scan: Scanout,
    frames: ModelFrames,
    (panel, modeset): (Panel, Option<Modeset>),
    window: (u32, u32),
) -> Head<ModelFrames> {
    let (bases, generation, flags, cursor, plug) = (frames.iova, 1, SCANNING_OUT, None, Plug::In);
    Head { out, scan, frames, panel, modeset, bases, generation, flags, window, cursor, plug }
}

/// A shadow of the output's size, every pixel `px`.
pub fn shadow_bytes(px: u32) -> Vec<u8> {
    px.to_le_bytes().iter().copied().cycle().take(1920 * 1080 * 4).collect()
}

/// The rig's first, and only, output.
pub fn h0(d: &Rig) -> &Head<ModelFrames> {
    d.heads[0].as_ref().unwrap()
}

pub fn h0m(d: &mut Rig) -> &mut Head<ModelFrames> {
    d.heads[0].as_mut().unwrap()
}
