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

//! net.core's single-frame receive numbers each ask and moves to the next
//! number only on a frame read, so a driver keeping the frame it handed out
//! (nonos_mac::resend) is asked for it again after a reply came too late.

use std::sync::{Arc, Mutex};

use nonos_libc::{serial, set_responder};

use crate::device::rx::poll_frame;

/// A driver that records each asked number and answers per `frame`: a
/// one-byte frame, or no reply at all (the reply came too late).
fn driver(frames: Vec<Option<u8>>) -> Arc<Mutex<Vec<u32>>> {
    let asked = Arc::new(Mutex::new(Vec::new()));
    let (log, mut frames) = (asked.clone(), frames.into_iter());
    set_responder(move |_, req| {
        log.lock().unwrap().push(u32::from_le_bytes(req[20..24].try_into().unwrap()));
        let Some(Some(b)) = frames.next() else { return Vec::new() };
        let mut r = req[..20].to_vec();
        r[16..20].copy_from_slice(&9u32.to_le_bytes());
        r.extend_from_slice(&0i32.to_le_bytes());
        r.extend_from_slice(&1u32.to_le_bytes());
        r.push(b);
        r
    });
    asked
}

#[test]
fn a_late_reply_is_asked_for_again_and_a_frame_moves_on() {
    let _g = serial();
    let asked = driver(vec![None, Some(7), Some(8)]);
    assert_eq!(poll_frame(3), None);
    assert_eq!(poll_frame(3), Some(vec![7]));
    assert_eq!(poll_frame(3), Some(vec![8]));
    let asked = asked.lock().unwrap().clone();
    assert_eq!(asked[1], asked[0], "no frame read: the same number again");
    assert_eq!(asked[2], asked[1].wrapping_add(1).max(1), "a frame read: the next number");
}
