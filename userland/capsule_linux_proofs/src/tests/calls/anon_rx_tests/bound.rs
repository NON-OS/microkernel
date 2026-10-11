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

//! The buffer holds one reply at most, and a longer fill is broken.

use super::super::super::random::Regs;
use super::held::{open, POLLIN};
use crate::linux::abi::errno::EIO;
use crate::linux::net::anon_ops::PAYLOAD_MAX;
use crate::linux::net::sock::{End, Got};

#[test]
fn the_buffer_never_holds_more_than_one_reply() {
    let mut r = Regs::new(0x5eed_0a11);
    for _ in 0..2000 {
        let mut a = open();
        for _ in 0..12 {
            let len = match r.small(4) {
                0 => PAYLOAD_MAX,
                1 => PAYLOAD_MAX + 1 + r.small(64) as usize,
                _ => r.small(PAYLOAD_MAX as u64 + 1) as usize,
            };
            match r.small(5) {
                0 => a.fill(Got::Bytes(vec![0x55; len])),
                1 => a.fill(Got::Nothing),
                2 => {
                    let _ =
                        a.take(r.small(PAYLOAD_MAX as u64 * 2) as usize, r.small(2) == 0, false);
                }
                3 => a.fill(Got::End(r.small(256) as u8)),
                _ => {
                    let _ = a.take(len, false, false);
                }
            }
            assert!(a.rx.len() <= PAYLOAD_MAX, "{} held", a.rx.len());
            if a.rx.is_empty() && a.end.is_none() {
                assert_eq!(a.bits(false, false) & POLLIN, 0, "readable on an empty open stream");
            }
        }
    }
}

#[test]
fn a_fill_longer_than_a_reply_is_a_broken_answer() {
    let mut a = open();
    a.fill(Got::Bytes(vec![0; PAYLOAD_MAX + 1]));
    assert!(a.rx.is_empty());
    assert_eq!(a.end, Some(End::Error(EIO)));
    assert_eq!(a.take(1, false, false), Err(EIO));
    assert_eq!(a.take(1, false, false), Ok(Vec::new()), "reported once, then end of file");
    let mut a = open();
    a.fill(Got::Bytes(vec![0; PAYLOAD_MAX]));
    assert_eq!(a.rx.len(), PAYLOAD_MAX, "a whole reply is taken");
}
