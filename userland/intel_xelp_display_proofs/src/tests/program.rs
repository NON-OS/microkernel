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
//! Writing a timing: everything checked before the first write, and a
//! transcoder that is on is never written.

use crate::model::mode::{cta_1080p, timing};
use crate::model::regs::Regs;
use nonos_intel_display_hw::pipe::{Pipe, Transcoder};
use nonos_intel_xelp_display::timing::check::Sink;
use nonos_intel_xelp_display::timing::write::program;

#[test]
fn writes_seven_registers_of_its_transcoder_and_pipe() {
    let mut m = Regs::new(Some(0));
    program(&mut m, Transcoder::C, Pipe::C, &cta_1080p(), Sink::Hdmi).unwrap();
    let regs: Vec<u32> = m.writes.iter().map(|w| w.0).collect();
    // Transcoder C's six timing registers, then pipe C's source size.
    assert_eq!(regs, [0x62000, 0x62004, 0x62008, 0x6200C, 0x62010, 0x62014, 0x6201C]);
    assert_eq!(m.get(0x62000), (2199 << 16) | 1919);
    assert_eq!(m.reads, [0x72008]);
}

#[test]
fn bad_timing_writes_nothing_and_reads_nothing() {
    let mut m = Regs::new(Some(0));
    let t = timing(1, [63, 32, 1, 1], [1, 5, 1, 1]);
    assert!(program(&mut m, Transcoder::A, Pipe::A, &t, Sink::DisplayPort).is_err());
    assert!(m.writes.is_empty() && m.reads.is_empty());
}

#[test]
fn edp_transcoder_writes_nothing() {
    let mut m = Regs::new(Some(0));
    assert!(program(&mut m, Transcoder::Edp, Pipe::A, &cta_1080p(), Sink::Hdmi).is_err());
    assert!(m.writes.is_empty() && m.reads.is_empty());
}

#[test]
fn transcoder_on_or_still_running_is_not_written() {
    // Enable bit set, or the state bit still set after the enable cleared.
    for conf in [0x8000_0000u32, 0x4000_0000, 0xC000_0000] {
        let mut m = Regs::new(None);
        m.set(0x70008, conf);
        assert!(program(&mut m, Transcoder::A, Pipe::A, &cta_1080p(), Sink::Hdmi).is_err());
        assert!(m.writes.is_empty(), "wrote into a live transcoder: {conf:#x}");
    }
}
