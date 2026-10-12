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

//! A request the device answers late. The wait gives up by the
//! clock and the request stays the device's: its buffer is never served and
//! nothing is posted over it, so whatever the device does afterwards no
//! entropy byte goes out twice. The driver used to post the next request
//! over it and take the late completion for the new one's: one completion
//! behind, it handed two callers the same bytes.

use super::drive::answered;
use super::model::{device, regs};
use super::ring::{Ring, SIZE};
use crate::clock::expired_after;
use crate::constants::FILL_MS;
use crate::fill::fill;
use crate::queue::Fault;
use crate::transport::Transport;

#[test]
fn a_request_that_timed_out_is_never_served_nor_posted_over() {
    let (bar, ring) = (device(SIZE), Ring::new());
    let t = Transport::Legacy(regs(&bar));
    let mut q = ring.queue();
    let mut out = [0u8; 64];
    assert_eq!(fill(t, &mut q, &mut out, expired_after(FILL_MS)), Err(Fault::TimedOut));
    assert!(q.stalled(), "the request is still the device's");
    ring.answer(0xA1);
    assert_eq!(fill(t, &mut q, &mut out, expired_after(FILL_MS)), Err(Fault::Stalled));
    assert_eq!(ring.avail(None), 1, "nothing was posted over it");
    assert_eq!(out, [0; 64], "its bytes were never served");
}

#[test]
fn no_byte_is_served_twice_across_a_late_completion() {
    let bar = device(SIZE);
    let t = Transport::Legacy(regs(&bar));
    let ring = Ring::new();
    let mut q = ring.queue();
    let mut served = vec![answered(t, &mut q, &ring, 0x10).expect("a live device")];
    // Request A is slow: the wait gives up, then the device answers it.
    assert_eq!(fill(t, &mut q, &mut [0; 64], expired_after(FILL_MS)), Err(Fault::TimedOut));
    ring.answer(0xA0);
    // B is never posted, so A's completion cannot pass for it.
    assert_eq!(answered(t, &mut q, &ring, 0xB0), Err(Fault::Stalled));
    assert_eq!(ring.avail(None), 2);
    // The server resets the device and brings it up on fresh grants.
    let fresh = Ring::new();
    let mut q = fresh.queue();
    for byte in [0xC0, 0xC1, 0xC2] {
        served.push(answered(t, &mut q, &fresh, byte).expect("served"));
    }
    for (i, a) in served.iter().enumerate() {
        assert_ne!(*a, [0xA0; 64], "the timed-out request's bytes went out");
        assert!(served[i + 1..].iter().all(|b| b != a), "fill {i}'s bytes went out twice");
    }
}
