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

//! Senders streaming after a recovery the kernel's own read set off, each
//! read queued as the reply to its last one goes, so every one lands in the
//! 1 ms look and the inbox is never seen empty. Any other sender the bound
//! served ends it with its next read (nvm::served): by then every request
//! queued while the recovery ran was taken. Kept, the bound refused the
//! stream from 33 s on, on a healthy drive, and cut a fresh kernel read
//! short with an Abort to the controller.

use super::after::{after_a_recovery, streamed};
use super::inbox::Inbox;

const INSTALLER: u32 = 42;
const OTHER: u32 = 43;

#[test]
fn an_installer_streaming_for_90_s_after_a_kernel_recovery_is_served() {
    let (mut s, first) = after_a_recovery(0);
    let mut inbox = Inbox::from([(first.at, INSTALLER)]);
    assert_eq!(streamed(&mut s, &mut inbox, first.at, 90_000), None, "refused when healthy");
}

#[test]
fn two_senders_streaming_in_turn_are_served_too() {
    // Each is the other's next in the inbox: the first one's next ends it.
    let (mut s, first) = after_a_recovery(0);
    let mut inbox = Inbox::from([(first.at, INSTALLER), (first.at, OTHER)]);
    assert_eq!(streamed(&mut s, &mut inbox, first.at, 90_000), None, "refused when healthy");
}

#[test]
fn a_fresh_kernel_read_inside_the_stream_has_its_own_time() {
    // 32 s on, the kernel queues a read that takes 2 s: it gets them, and
    // no Abort goes to the healthy controller.
    let (mut s, first) = after_a_recovery(0);
    let (mut inbox, from) =
        (Inbox::from([(first.at, INSTALLER)]), s.fake.borrow().admin_seen.len());
    assert_eq!(streamed(&mut s, &mut inbox, first.at, 32_000), None);
    let now = s.fake.borrow().ticks * 100;
    inbox.push_back((now, 0));
    s.fake.borrow_mut().hold = 20;
    let k = s.serve_next(&mut inbox).expect("queued");
    assert_eq!((k.status, s.ops_since(from)), (Ok(()), vec![]), "cut at {} ms", k.ms);
}
