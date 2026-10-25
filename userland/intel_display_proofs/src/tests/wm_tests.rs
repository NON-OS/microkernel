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

//! Watermark levels for a linear plane, and their registers.

use crate::engine::ddb::Run;
use crate::engine::pipe::Pipe;
use crate::engine::wm::{levels, Level};
use crate::engine::wm_write::{cur_buf_cfg, plane_buf_cfg, plane_wm, write};
use crate::model::Gen9;

const LAT: [u32; 8] = [2, 4, 8, 16, 24, 32, 48, 0];

#[test]
fn ten_eighty_p_levels_follow_the_two_methods() {
    let l = levels(&LAT, 1920, 2200, 148_500, 864);
    // Level 0, 2 us: method 1, 3 blocks, beats method 2's whole line.
    assert_eq!(l[0], Level { enabled: true, lines: 1, blocks: 4 });
    // Level 6, 48 us: method 1's 56 blocks still beat method 2's four
    // whole lines of 15 blocks; 56 blocks span four lines.
    assert_eq!(l[6], Level { enabled: true, lines: 4, blocks: 57 });
    assert!(!l[7].enabled, "no latency, no level");
}

#[test]
fn a_level_too_big_for_the_share_turns_off_with_those_above() {
    let l = levels(&LAT, 3840, 4000, 533_250, 60);
    assert!(l[0].enabled);
    let first_off = l.iter().position(|x| !x.enabled).unwrap();
    assert!(l[first_off..].iter().all(|x| !x.enabled));
    assert!(l.iter().filter(|x| x.enabled).all(|x| x.blocks < 60 && x.lines <= 31));
    assert!(levels(&LAT, 0, 2200, 148_500, 864).iter().all(|x| !x.enabled));
}

#[test]
fn the_registers_take_the_runs_and_levels() {
    let mut m = Gen9::new(1);
    let l = levels(&LAT, 1920, 2200, 148_500, 864);
    write(&mut m, Pipe::B, (Run { start: 0, end: 864 }, Run { start: 864, end: 896 }), &l);
    assert_eq!(m.get(plane_buf_cfg(Pipe::B)), (863 << 16));
    assert_eq!(m.get(plane_wm(Pipe::B, 0)), (1 << 31) | (1 << 14) | 4);
    assert_eq!(m.get(cur_buf_cfg(Pipe::B)), (895 << 16) | 864);
    assert_eq!(plane_wm(Pipe::B, 0), 0x7_1240);
}
