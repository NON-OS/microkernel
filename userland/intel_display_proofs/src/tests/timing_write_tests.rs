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

//! Writing a transcoder's timing, read back by the takeover's reader.

use crate::engine::error::Error;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::{regs, timing, timing_write};
use crate::model::Gen9;
use nonos_edid::Detailed;

fn uhd() -> Detailed {
    Detailed {
        pixel_khz: 533_250,
        h_active: 3840,
        h_blank: 160,
        h_front: 48,
        h_sync: 32,
        v_active: 2160,
        v_blank: 62,
        v_front: 3,
        v_sync: 5,
        h_positive: true,
        v_positive: false,
        interlaced: false,
        width_mm: 0,
        height_mm: 0,
    }
}

#[test]
fn a_written_timing_reads_back_whole() {
    let mut m = Gen9::new(1);
    timing_write::write(&mut m, Transcoder::B, Pipe::B, &uhd()).unwrap();
    let back = timing::read(&mut m, Transcoder::B).unwrap();
    let t = uhd();
    assert_eq!((back.h_active, back.h_blank, back.h_front, back.h_sync), (3840, 160, 48, 32));
    assert_eq!((back.v_active, back.v_blank, back.v_front, back.v_sync), (2160, 62, 3, 5));
    assert_eq!(m.get(regs::pipe_src(Pipe::B)), (3839 << 16) | 2159);
    let _ = t;
}

#[test]
fn a_timing_the_fields_cannot_hold_is_refused_unwritten() {
    let mut m = Gen9::new(1);
    let mut wide = uhd();
    wide.h_active = 8100;
    let mut late = uhd();
    late.h_front = 150;
    let mut inter = uhd();
    inter.interlaced = true;
    for t in [wide, late, inter] {
        assert_eq!(timing_write::write(&mut m, Transcoder::A, Pipe::A, &t), Err(Error::Argument));
    }
    assert!(m.writes.is_empty());
}
