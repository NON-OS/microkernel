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
//! TRANS_CONF off, and a transcoder that is not there.

use crate::model::clock::Steps;
use crate::model::regs::Regs;
use nonos_intel_display_hw::pipe::Transcoder;
use nonos_intel_xelp_display::transcoder::{disable, enable};

const FRAME: u64 = 16_667;

#[test]
fn disable_clears_31_and_waits_for_30() {
    let mut m = Regs::new(Some(2));
    m.set(0x73008, 0xC000_0024);
    disable(&mut m, &Steps::new(10), Transcoder::D, FRAME).unwrap();
    assert_eq!(m.writes, [(0x73008, 0x4000_0024)]);
    assert_eq!(m.get(0x73008), 0x24);
}

#[test]
fn disable_that_stays_running_times_out() {
    let mut m = Regs::new(None);
    m.set(0x70008, 0xC000_0000);
    assert!(disable(&mut m, &Steps::new(1000), Transcoder::A, FRAME).is_err());
}

#[test]
fn edp_refused_before_any_access() {
    let mut m = Regs::new(Some(0));
    assert!(enable(&mut m, &Steps::new(1), Transcoder::Edp, FRAME).is_err());
    assert!(disable(&mut m, &Steps::new(1), Transcoder::Edp, FRAME).is_err());
    assert!(m.reads.is_empty() && m.writes.is_empty());
}
