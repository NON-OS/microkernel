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

use crate::engine::gen::GEN9;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::scanout::Scanout;
use crate::engine::takeover::read_output;
use crate::model::firmware::T1080;
use crate::model::{Gen9, Polls};

pub(super) const TOTAL: u32 = 1 << 20;

pub(super) fn setup() -> (Gen9, Vec<u64>, Vec<u64>) {
    let mut m = Gen9::new(TOTAL as usize);
    m.light(Transcoder::Edp, Pipe::A, T1080, 4 << 12);
    m.fill(0x9000_0000, 1, 0xFF00_0001);
    crate::engine::ggtt::map(&mut m, 0, &[0x9000_0000; 2025], TOTAL).unwrap();
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
    let mut s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], &Polls::new(50)).unwrap();
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
