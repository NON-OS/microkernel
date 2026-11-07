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

//! While SERVE_ONE serves, its pump is the inbox's one reader and keeps
//! every frame that owes no answer for the input pump. What is kept comes
//! back whole, from the sender the kernel named, in the order it came; what
//! does not fit is refused, never cut.

#[path = "../../../../capsule_linux/src/linux/net/serve_one/aside_queue.rs"]
mod aside_queue;

use aside_queue::{Aside, KEPT_MAX};

const CAP: usize = 48;
const ROUTER: u32 = 3;

fn any(_: u32) -> bool {
    true
}

fn router(sender: u32) -> bool {
    sender == ROUTER
}

#[test]
fn frames_come_back_in_order_with_their_senders() {
    let mut a = Aside::new();
    assert!(a.put(7, &[1; 40], CAP, any));
    assert!(a.put(9, &[2; 28], CAP, any));
    let mut rx = [0u8; CAP];
    assert_eq!(a.take(&mut rx), Some((40, 7)));
    assert_eq!(&rx[..40], &[1; 40]);
    assert_eq!(a.take(&mut rx), Some((28, 9)));
    assert_eq!(&rx[..28], &[2; 28]);
    assert_eq!(a.take(&mut rx), None);
}

#[test]
fn an_empty_or_overlong_frame_is_not_kept() {
    let mut a = Aside::new();
    assert!(!a.put(7, &[], CAP, any));
    assert!(!a.put(7, &[0; CAP + 1], CAP, any));
    assert!(a.put(7, &[0; CAP], CAP, any));
    assert_eq!(a.take(&mut [0u8; CAP]), Some((CAP, 7)));
}

#[test]
fn a_full_queue_refuses_the_newest_and_keeps_the_rest_in_order() {
    let mut a = Aside::new();
    for i in 0..KEPT_MAX {
        assert!(a.put(1, &[i as u8], CAP, any));
    }
    assert!(!a.put(1, &[0xFF], CAP, any));
    let mut rx = [0u8; CAP];
    for i in 0..KEPT_MAX {
        assert_eq!(a.take(&mut rx), Some((1, 1)));
        assert_eq!(rx[0], i as u8);
    }
    assert_eq!(a.take(&mut rx), None);
}

#[test]
fn a_frame_longer_than_the_reader_stays_kept() {
    let mut a = Aside::new();
    assert!(a.put(3, &[5; 40], CAP, any));
    assert_eq!(a.take(&mut [0u8; 39]), None);
    assert_eq!(a.take(&mut [0u8; 40]), Some((40, 3)));
}

#[test]
fn a_flood_from_a_sender_not_believed_takes_no_room() {
    let mut a = Aside::new();
    for i in 0..4 * KEPT_MAX {
        assert!(!a.put(40, &[i as u8], CAP, router));
    }
    for i in 0..KEPT_MAX {
        assert!(a.put(ROUTER, &[i as u8], CAP, router));
    }
    assert!(!a.put(ROUTER, &[0xFF], CAP, router));
    let mut rx = [0u8; CAP];
    for i in 0..KEPT_MAX {
        assert_eq!(a.take(&mut rx), Some((1, ROUTER)));
        assert_eq!(rx[0], i as u8);
    }
    assert_eq!(a.take(&mut rx), None);
}
