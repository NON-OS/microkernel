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

use super::resend::{serve, Kept};

/// A driver queue: each fresh call takes the next byte as a one-byte frame.
fn take(queue: &mut Vec<u8>) -> impl FnOnce(&mut [u8]) -> Result<Option<usize>, ()> + '_ {
    move |buf| {
        Ok(if queue.is_empty() {
            None
        } else {
            buf[0] = queue.remove(0);
            Some(1)
        })
    }
}

fn one(kept: &mut Kept<4>, seq: Option<u32>, queue: &mut Vec<u8>) -> Option<u8> {
    serve(kept, seq, take(queue)).unwrap().map(|f| f[0])
}

#[test]
fn a_reask_after_a_lost_reply_gets_the_same_frame() {
    let (mut kept, mut q) = (Kept::<4>::new(), vec![1, 2]);
    assert_eq!(one(&mut kept, Some(7), &mut q), Some(1));
    // The reply was lost: net.core asks 7 again.
    assert_eq!(one(&mut kept, Some(7), &mut q), Some(1));
    assert_eq!(one(&mut kept, Some(8), &mut q), Some(2));
    assert!(q.is_empty(), "no frame dropped, none taken twice");
}

#[test]
fn nothing_queued_keeps_nothing() {
    let (mut kept, mut q) = (Kept::<4>::new(), vec![]);
    assert_eq!(one(&mut kept, Some(7), &mut q), None);
    q.push(5);
    assert_eq!(one(&mut kept, Some(7), &mut q), Some(5));
}

#[test]
fn the_old_wire_takes_a_fresh_frame_every_time() {
    let (mut kept, mut q) = (Kept::<4>::new(), vec![1, 2]);
    assert_eq!(one(&mut kept, None, &mut q), Some(1));
    assert_eq!(one(&mut kept, None, &mut q), Some(2));
}

#[test]
fn asking_a_new_number_lets_the_old_frame_go() {
    let (mut kept, mut q) = (Kept::<4>::new(), vec![1]);
    assert_eq!(one(&mut kept, Some(7), &mut q), Some(1));
    assert_eq!(one(&mut kept, Some(8), &mut q), None);
    q.push(2);
    assert_eq!(one(&mut kept, Some(7), &mut q), Some(2), "7 was let go when 8 was asked");
}
