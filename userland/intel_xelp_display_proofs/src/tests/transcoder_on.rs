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
//! TRANS_CONF on: the bits written, the wait, and what a
//! transcoder that never answers is left as.

use crate::model::clock::{Dead, Steps};
use crate::model::regs::Regs;
use nonos_intel_display_hw::pipe::Transcoder;
use nonos_intel_xelp_display::transcoder::enable;

const CONF_B: u32 = 0x71008;
const FRAME: u64 = 16_667;

#[test]
fn enable_sets_31_clears_interlace_keeps_the_rest() {
    let mut m = Regs::new(Some(3));
    // Interlaced mode 22:21 left set, and the audio watermark 6:0 at 24h.
    m.set(CONF_B, (3 << 21) | 0x24);
    enable(&mut m, &Steps::new(10), Transcoder::B, FRAME).unwrap();
    assert_eq!(m.writes, [(CONF_B, 0x8000_0024)]);
    assert_eq!(m.get(CONF_B), 0xC000_0024);
}

#[test]
fn enable_that_never_lights_is_turned_back_off() {
    let mut m = Regs::new(None);
    let c = Steps::new(1000);
    assert!(enable(&mut m, &c, Transcoder::B, FRAME).is_err());
    assert_eq!(m.get(CONF_B) & 0x8000_0000, 0);
    // It waited two frames, and not much more.
    assert!(c.now() >= 2 * FRAME && c.now() <= 2 * FRAME + 3000, "{}", c.now());
}

#[test]
fn dead_clock_is_a_timeout_and_off() {
    let mut m = Regs::new(None);
    assert!(enable(&mut m, &Dead, Transcoder::A, FRAME).is_err());
    assert_eq!(m.get(0x70008) & 0x8000_0000, 0);
}

#[test]
fn state_landing_at_the_deadline_still_counts() {
    // With the clock a frame a read, the wait reads the register twice: the
    // second read is past the deadline, and a state landing there counts.
    let mut m = Regs::new(Some(1));
    assert!(enable(&mut m, &Steps::new(FRAME), Transcoder::A, FRAME).is_ok());
    // One read later is too late.
    let mut m = Regs::new(Some(2));
    assert!(enable(&mut m, &Steps::new(FRAME), Transcoder::A, FRAME).is_err());
}
