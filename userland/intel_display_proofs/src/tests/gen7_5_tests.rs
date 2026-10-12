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

//! The takeover on Haswell: Broadwell's plane, and the GGTT's 32-bit
//! entries from 2 MiB, sized by GGMS in bits 9:8.

use super::scanout_tests::setup;
use crate::engine::error::Error;
use crate::engine::gen::{PlaneKind, Pte, GEN7_5};
use crate::engine::ggtt::{map, unmap};
use crate::engine::ggtt_range::entries;
use crate::engine::pipe::Pipe;
use crate::engine::{plane_regs as r, scanout::Scanout, takeover::read_output};
use crate::model::{Gen9, Polls};

#[test]
fn narrow_entries_are_one_32_bit_write_each_and_flushed() {
    assert_eq!(Pte::Narrow.encode(0x12_3456_7000), 0x3456_7121, "39:32 in 11:4");
    let mut m = Gen9::new(16);
    map(&mut m, Pte::Narrow, 2, &[0x1000, 0x12_3456_7000], 16).unwrap();
    let narrow: Vec<_> =
        m.writes.iter().filter(|w| (0x20_0000..0x40_0000).contains(&w.0)).collect();
    assert_eq!(narrow.len(), 0, "the model takes them before its register log");
    assert_eq!(&m.ggtt[2..4], &[0x1001, 0x12_3456_7001], "widened back by the model");
    assert_eq!(m.flushes, 1);
    unmap(&mut m, Pte::Narrow, 2, 1, 16).unwrap();
    assert_eq!(&m.ggtt[2..4], &[0, 0x12_3456_7001]);
    let far = map(&mut m, Pte::Narrow, 0, &[0x100_0000_0000], 16);
    assert_eq!(far, Err(Error::Argument), "past 40 bits");
}

#[test]
fn haswells_table_size_is_in_bits_9_8() {
    assert_eq!(entries(Pte::Narrow, 0), 0);
    assert_eq!(entries(Pte::Narrow, 1 << 8), 262_144, "1 MiB of 4 byte entries");
    assert_eq!(entries(Pte::Narrow, 2 << 8), 524_288);
    assert_eq!(entries(Pte::Narrow, 3 << 8), 0, "reserved");
    assert_eq!(entries(Pte::Narrow, 3 << 6), 0, "Broadwell's field is not read");
}

/// A Haswell table of 2 MiB: 512 Ki entries, all inside 2 to 4 MiB.
const TOTAL: u32 = 524_288;

#[test]
fn start_present_and_stop_on_haswell() {
    let (mut m, a, b) = setup();
    m.plane = PlaneKind::Primary;
    m.set(r::plane_ctl(Pipe::A, 1), (1 << 31) | (0x6 << 26));
    m.set(r::plane_stride(Pipe::A, 1), 7680);
    let out = read_output(&mut m, &GEN7_5, Pipe::A).unwrap();
    assert_eq!(out.ggtt, Pte::Narrow);
    let mut s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_00AA));
    s.present(&mut m, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFF00_00BB));
    let runs = s.surfaces;
    s.stop(&mut m, TOTAL, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF00_0001), "firmware's picture back");
    assert!(runs.iter().all(|&g| m.ggtt[(g / 4096) as usize] == 0), "our entries gone");
    assert!(m.writes.iter().all(|w| w.0 < 0x20_0000), "no wide entry written");
}
