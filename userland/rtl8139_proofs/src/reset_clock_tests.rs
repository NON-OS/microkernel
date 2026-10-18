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

//! The software reset's wait is bounded by the clock, not by a count of
//! reads: a part that never leaves reset and answers each read slowly, as a
//! loaded machine or a slow port path does, is given up on once the bound
//! has passed, not after a million reads.

use std::time::{Duration, Instant};

use nonos_libc::{attach, uptime_ms, Port};

use crate::init::reset_part;
use crate::part::{part, Model};
use crate::pio::Pio;

/// The part, each read taking a millisecond.
struct Slow(Model);

impl Port for Slow {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32> {
        std::thread::sleep(Duration::from_millis(1));
        self.0.read(offset, width)
    }

    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool {
        self.0.write(offset, width, value)
    }
}

#[test]
fn a_slow_part_stuck_in_reset_is_given_up_on_within_the_bound() {
    let stuck =
        Model { regs: [0; 256], stuck_in_reset: true, refuse: None, resets: Default::default() };
    let _port = attach(Box::new(Slow(stuck)));
    let start = Instant::now();
    assert_eq!(reset_part(&Pio::new(3)), Err("rtl8139 reset timeout"));
    let took = start.elapsed();
    assert!(took >= Duration::from_millis(10), "the whole bound was waited: {took:?}");
    assert!(took < Duration::from_millis(500), "and not a read count's worth more: {took:?}");
}

/// The uptime clock counts whole milliseconds, so a deadline taken late in
/// one can fall due a millisecond short; begun there, the bound still holds.
#[test]
fn a_reset_begun_late_in_a_millisecond_still_waits_the_whole_bound() {
    let _part = part(true);
    let tick = uptime_ms();
    while uptime_ms() == tick {
        std::hint::spin_loop();
    }
    let edge = Instant::now();
    while edge.elapsed() < Duration::from_micros(900) {
        std::hint::spin_loop();
    }
    let start = Instant::now();
    assert_eq!(reset_part(&Pio::new(3)), Err("rtl8139 reset timeout"));
    let took = start.elapsed();
    assert!(took >= Duration::from_millis(10), "the whole bound was waited: {took:?}");
}
