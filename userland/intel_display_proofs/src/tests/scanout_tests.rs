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

//! The takeover's whole life on the model: start on firmware's mode,
//! present frames by flipping, hand the output back.

use crate::engine::access::Mmio;
use crate::engine::flip::flip;
use crate::engine::gen::GEN9;
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_regs as r;
use crate::engine::scanout::Scanout;
use crate::engine::takeover::read_output;
use crate::model::firmware::T1080;
use crate::model::{Gen9, Polls};

pub(super) const TOTAL: u32 = 1 << 20;

pub(super) fn setup() -> (Gen9, Vec<u64>, Vec<u64>) {
    let mut m = Gen9::new(TOTAL as usize);
    m.light(Transcoder::Edp, Pipe::A, T1080, 4 << 12);
    m.fill(0x9000_0000, 1, 0xFF00_0001);
    crate::engine::ggtt::map(&mut m, Pte::Wide, 0, &[0x9000_0000; 2025], TOTAL).unwrap();
    m.fill(0x1000_0000, 2025, 0xFF00_00AA);
    m.fill(0x2000_0000, 2025, 0xFF00_00BB);
    let a = (0..2025).map(|i| 0x1000_0000 + i * 4096).collect();
    let b = (0..2025).map(|i| 0x2000_0000 + i * 4096).collect();
    (m, a, b)
}

#[test]
fn start_present_and_stop_on_firmwares_mode() {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, &GEN9, Pipe::A).unwrap();
    assert_eq!(m.pixel(Pipe::A, 7, 7), Some(0xFF00_0001), "firmware's picture");
    let mut s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], u32::MAX, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_00AA));
    assert!(s.surfaces.iter().all(|&g| g % (256 * 1024) == 0 && g / 4096 >= 2025));
    assert_eq!(s.back(), 1);
    s.present(&mut m, &Polls::new(50)).unwrap();
    assert_eq!((m.pixel(Pipe::A, 0, 0), s.back()), (Some(0xFF00_00BB), 0));
    s.present(&mut m, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFF00_00AA));
    let runs = s.surfaces;
    s.stop(&mut m, TOTAL, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF00_0001), "firmware's picture back");
    assert!(runs.iter().all(|&g| m.ggtt[(g / 4096) as usize] == 0), "our entries gone");
}

/// A flip given up at its deadline, the pipe never blanking: the plane is
/// pointed at the front again, so a later blank cannot latch the back while
/// the next frame is drawn into it, and the back stays the back.
#[test]
fn a_flip_given_up_points_the_plane_at_the_front_again() {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, &GEN9, Pipe::A).unwrap();
    let mut s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], u32::MAX, &Polls::new(50)).unwrap();
    let (front, back) = (s.surfaces[s.front], s.surfaces[s.back()]);
    m.period = 0;
    assert_eq!(s.present(&mut m, &Polls::new(20)), Err(crate::engine::error::Error::Timeout));
    assert_eq!(m.get(r::plane_surf(Pipe::A, 1)), front, "no flip left armed");
    assert_eq!(s.surfaces[s.back()], back, "the frame is drawn into the back again");
    // The pipe blanks again: the front stays on screen, the back unseen.
    m.period = 3;
    for _ in 0..9 {
        let _ = m.read(r::plane_surflive(Pipe::A, 1));
    }
    assert_eq!(m.get(r::plane_surflive(Pipe::A, 1)) & !0xFFF, front);
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFF00_00AA));
}

/// A started scanout whose pipe blanks every `period` reads, its back.
fn started(period: u32) -> ((Gen9, u32), Scanout) {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, &GEN9, Pipe::A).unwrap();
    let s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], u32::MAX, &Polls::new(50)).unwrap();
    m.period = period;
    let back = s.surfaces[s.back()];
    ((m, back), s)
}

/// A flip that landed just past its deadline counts as landed: the back
/// is shown, so it is the front from now on. On this rig a pipe blanking
/// every third read gives the flip alone up after one poll while the plane
/// shows the back on the next read.
#[test]
fn a_flip_that_landed_past_its_deadline_counts_as_landed() {
    let ((mut alone, back), _) = started(3);
    assert!(flip(&mut alone, Pipe::A, back, &Polls::new(1)).is_err(), "given up");
    let ((mut m, _), mut s) = started(3);
    assert_eq!(s.present(&mut m, &Polls::new(1)), Ok(()));
    assert_eq!(m.get(r::plane_surflive(Pipe::A, 1)) & !0xFFF, back);
    assert_eq!(s.surfaces[s.front], back, "the back is the front now");
}
