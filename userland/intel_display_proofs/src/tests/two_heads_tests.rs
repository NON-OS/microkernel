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

//! Two outputs on one engine (PLAN G 10): each its own wire output, its
//! surfaces in its own GGTT window, its FLUSH shown on its own pipe only;
//! an output past them is ENXIO.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::{Mode, Rect};

use super::two_rig::two_rig;
use crate::engine::pipe::Pipe;

#[test]
fn each_lit_pipe_is_an_output_and_none_past_them() {
    let (mut d, _, _) = two_rig();
    assert_eq!(d.caps().outputs, 2);
    let mut out = [Mode::default(); 2];
    d.modes(1, &mut out).unwrap();
    assert_eq!((out[0].width, out[0].height), (1024, 768));
    assert_eq!(d.state(1).map(|s| (s.width, s.height)), Ok((1024, 768)));
    assert_eq!(d.modes(2, &mut out), Err(ENXIO));
    assert_eq!(d.state(2).err(), Some(ENXIO));
}

#[test]
fn every_outputs_surfaces_stay_in_its_window() {
    let (d, _, windows) = two_rig();
    for (h, w) in d.heads.iter().flatten().zip(windows) {
        let runs = h.scan.surfaces.map(|g| g / 4096);
        assert!(runs.iter().all(|&r| r >= w.0 && r + h.scan.pages <= w.1), "{runs:?} {w:?}");
    }
}

#[test]
fn a_flush_shows_on_its_own_pipe_only() {
    let (mut d, model, _) = two_rig();
    let px: Vec<u8> =
        0xFF12_3456u32.to_le_bytes().iter().copied().cycle().take(1024 * 768 * 4).collect();
    let s = Shadow { bytes: &px, width: 1024, height: 768, stride: 4096, cursor_offset: 0 };
    d.flush(1, &[Rect { x: 0, y: 0, width: 16, height: 16 }], &s).unwrap();
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::B, 3, 3), Some(0xFF12_3456));
    assert_eq!(m.pixel(Pipe::A, 3, 3), Some(0xFF00_00AA), "pipe A untouched");
    assert_eq!(d.flush(2, &[], &s), Err(ENXIO));
}
