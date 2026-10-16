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

//! A controller that writes anything at all into the event ring.
//!
//! Every round posts up to four events whose every dword is random, with the
//! fields that steer the driver drawn half the time from the values that
//! matter: the type from transfer, command, port change, 0 and 63; the
//! pointer from the issued TRB itself, the same TRB with reserved bits set,
//! its neighbours, the high half flipped, null, below 16 and all ones; the
//! slot from the configured one, one never enabled, 0 and 255; the endpoint
//! from the configured one, endpoint 1, 0 and 31; the third dword from all
//! ones and residuals at and past the request. One of the driver's
//! consumers then takes the ring: the drain, the interrupt-IN poll, a bulk
//! transfer, a control transfer or an Enable Slot command. The waits get
//! their genuine event posted behind the noise, so each round ends. Some
//! rounds leave a half-written TRB, its cycle bit the wrong way, at the
//! enqueue slot.
//!
//! After every round nothing has panicked, the dequeue pointer and ERDP lie
//! in the segment on a TRB boundary with DESI zero, ERDP names the next
//! unread TRB, no TRB from past the segment was read, every length handed up
//! is within its request and its buffer, every slot id taken from an answer
//! is refused unless the slot table can hold it, and the producer never had
//! to write over an event the driver had not handed back.

use super::driver::{Driver, ROUND_ROOM};
use super::genuine::random_trb;
use super::rng::Rng;

const ROUNDS: u32 = 200_000;

impl Driver {
    fn round(&mut self, rng: &mut Rng) {
        if self.fx.hc.room() < ROUND_ROOM {
            self.flush();
        }
        match rng.below(5) {
            0 => self.drain(rng),
            1 => self.poll(rng),
            2 => self.bulk(rng),
            3 => self.control(rng),
            _ => self.enable_slot(rng),
        }
        if rng.below(8) == 0 {
            let half_written = random_trb(rng);
            self.fx.hc.scribble(half_written);
        }
        self.check();
    }
}

#[test]
fn hostile_events_never_panic_or_reach_past_a_ring_or_a_buffer() {
    let mut driver = Driver::new();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..ROUNDS {
        driver.round(&mut rng);
    }
    let laps = driver.fx.ring.drained_total() / driver.fx.hc.segment_trbs() as u64;
    assert!(laps > 1000, "the rounds went round the ring {laps} times");
}
