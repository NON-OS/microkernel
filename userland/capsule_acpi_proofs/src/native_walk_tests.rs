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

//! A long walk of a slot through arbitrary polls, every capability mix:
//! a probe only after the card was up `READY_MS` without a break, a
//! departure only after it was gone `DEBOUNCE_MS` without a break, an
//! eject only while it is up, and power never switched without a
//! controller.

use crate::native::regs::{BUTTON_PRESSED, PRESENT};
use crate::native::slot::{Act, Read, Slot};
use crate::native::times::{DEBOUNCE_MS, READY_MS};

fn reads(x: u64) -> Read {
    let status =
        (if x & 1 != 0 { PRESENT } else { 0 }) | if x & 2 != 0 { BUTTON_PRESSED } else { 0 };
    let link = [Some(true), Some(false), None][(x >> 2) as usize % 3];
    Read { status, link, powered: x & 32 != 0 }
}

#[test]
fn every_act_is_earned() {
    for caps in 0..4u8 {
        let (button, power) = (caps & 1 != 0, caps & 2 != 0);
        let mut seed = 0x9E37_79B9_7F4A_7C15u64 ^ u64::from(caps);
        let mut s = Slot::new(button, power, reads(0));
        let (mut now, mut up_since, mut out_since) = (0u64, None::<u64>, Some(0u64));
        for _ in 0..200_000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            now += 5 + (seed >> 59);
            let r = reads(seed >> 32);
            if r.here() {
                up_since = up_since.or(Some(now));
                out_since = None;
            } else {
                out_since = out_since.or(Some(now));
                up_since = None;
            }
            match s.step(r, now) {
                Act::Probe => assert!(now - up_since.unwrap() >= READY_MS),
                Act::Depart => assert!(now - out_since.unwrap() >= DEBOUNCE_MS),
                Act::Eject => assert!(r.here()),
                Act::PowerOn | Act::PowerOff => assert!(power),
                Act::Nothing => {}
            }
            if seed & 0xFF == 0 && s.ejected() != Act::Nothing {
                assert!(power, "power off after an eject needs a controller");
            }
        }
    }
}
