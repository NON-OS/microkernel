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
//! Random timings, of the kind a hostile or broken EDID could carry: each
//! is refused with nothing written, or written with every word inside the
//! limits Vol 2c sets, decoded back here and checked on their own terms.

use crate::model::mode::timing;
use crate::model::regs::Regs;
use crate::model::rng::Rng;
use nonos_intel_display_hw::pipe::{Pipe, Transcoder};
use nonos_intel_xelp_display::timing::check::Sink;
use nonos_intel_xelp_display::timing::write::program;

fn hi(w: u32) -> u32 {
    w >> 16
}
fn lo(w: u32) -> u32 {
    w & 0xFFFF
}

#[test]
fn every_timing_is_refused_cleanly_or_written_in_bounds() {
    let mut r = Rng(0x9E37_79B9_7F4A_7C15);
    let (mut taken, mut refused) = (0, 0);
    for _ in 0..200_000 {
        let (h, v) = (r.side(), r.side());
        let sink = if r.word().is_multiple_of(2) { Sink::Hdmi } else { Sink::DisplayPort };
        let mut m = Regs::new(Some(0));
        let Ok(w) = program(&mut m, Transcoder::A, Pipe::A, &timing(r.field(), h, v), sink) else {
            assert!(m.writes.is_empty());
            refused += 1;
            continue;
        };
        taken += 1;
        // No field runs into its reserved bits (Vol 2c p. 1364, 1396, 704).
        for x in [w.htotal, w.hblank, w.hsync] {
            assert!(hi(x) < 1 << 14 && lo(x) < 1 << 14);
        }
        for x in [w.vtotal, w.vblank, w.vsync] {
            assert!(hi(x) < 1 << 13 && lo(x) < 1 << 13);
        }
        assert!(hi(w.srcsz) < 1 << 13 && lo(w.srcsz) < 4320);
        // Sync strictly between active and total, both ways (p. 1363, 1393).
        assert!(lo(w.hsync) > lo(w.htotal) && hi(w.hsync) > lo(w.hsync));
        assert!(hi(w.hsync) < hi(w.htotal));
        assert!(lo(w.vsync) > lo(w.vtotal) && hi(w.vsync) > lo(w.vsync));
        assert!(hi(w.vsync) < hi(w.vtotal));
        // Blank at least 32 pixels and 5 lines; active at least 64 pixels.
        assert!(hi(w.htotal) - lo(w.htotal) >= 32 && hi(w.vtotal) - lo(w.vtotal) >= 5);
        assert!(lo(w.htotal) >= 63 && lo(w.srcsz) == lo(w.vtotal));
        if sink == Sink::Hdmi {
            assert!(lo(w.htotal) >= 255 && lo(w.hsync) - lo(w.htotal) >= 16);
        }
    }
    assert!(taken > 1000 && refused > 1000, "sweep reached {taken} and {refused}");
}
