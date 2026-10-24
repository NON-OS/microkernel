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
//! Every register of every transcoder at the address Vol 2c lists for it.

use nonos_intel_display_hw::pipe::{Pipe, Transcoder};
use nonos_intel_xelp_display::regs::trans::{self, of, srcsz};

const T: [Transcoder; 4] = [Transcoder::A, Transcoder::B, Transcoder::C, Transcoder::D];

#[test]
fn timing_registers_at_their_listed_addresses() {
    // Vol 2c p. 1364, 1360, 1362, 1396, 1377, 1392, 1340: the A to D rows.
    let listed: [(u32, [u32; 4]); 7] = [
        (trans::HTOTAL, [0x60000, 0x61000, 0x62000, 0x63000]),
        (trans::HBLANK, [0x60004, 0x61004, 0x62004, 0x63004]),
        (trans::HSYNC, [0x60008, 0x61008, 0x62008, 0x63008]),
        (trans::VTOTAL, [0x6000C, 0x6100C, 0x6200C, 0x6300C]),
        (trans::VBLANK, [0x60010, 0x61010, 0x62010, 0x63010]),
        (trans::VSYNC, [0x60014, 0x61014, 0x62014, 0x63014]),
        (trans::CONF, [0x70008, 0x71008, 0x72008, 0x73008]),
    ];
    for (reg, at) in listed {
        for (t, want) in T.iter().zip(at) {
            assert_eq!(of(reg, *t), Ok(want));
        }
    }
}

#[test]
fn source_size_per_pipe() {
    // PIPE_SRCSZ, Vol 2c p. 704.
    let at = [0x6001C, 0x6101C, 0x6201C, 0x6301C];
    for (p, want) in Pipe::ALL.iter().zip(at) {
        assert_eq!(srcsz(*p), want);
    }
}

#[test]
fn no_edp_transcoder() {
    assert!(of(0x60000, Transcoder::Edp).is_err());
}

#[test]
fn conf_bits() {
    // TRANS_CONF: enable 31, state 30, interlaced mode 22:21 (p. 1341).
    assert_eq!(trans::CONF_ENABLE, 0x8000_0000);
    assert_eq!(trans::CONF_STATE, 0x4000_0000);
    assert_eq!(trans::CONF_INTERLACE, 0x0060_0000);
}
