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

//! The greeting goes only after a reset that reached the proxy. Both
//! resets refused, a guest's next stream on the same named stream sent its
//! greeting into the last stream's conversation: net.socks5 carried it to
//! the last far end and handed back what that end had sent, which was read
//! as the method reply ("net.socks5 sent an answer this cannot read").

use crate::carrier::{Carrier, NO_ROOM};
use crate::crowded::{after_last, EBUSY};
use crate::opening::Opening;
use crate::refusal::Proxy;
use crate::slice::Slice;
use crate::tunnel::Tunnel;

#[test]
fn a_blocking_open_greets_only_after_a_reset_reached_the_proxy() {
    let (fake, carrier) = after_last();
    let next = match Tunnel::open(carrier, Proxy::Nym, "n.example", 1) {
        Ok(t) => t,
        Err(why) => panic!("the next stream read the last one's bytes: {why}"),
    };
    assert!(next.pending.is_empty());
    assert_eq!(fake.0.borrow().resets, 2, "the next open's reset reached the proxy");
}

#[test]
fn a_sliced_open_greets_only_after_a_reset_reached_the_proxy() {
    let (fake, carrier) = after_last();
    let Ok(mut o) = Opening::begin(carrier, Proxy::Nym, "n.example", 1, 50) else { panic!() };
    let mut end = Slice::Waiting;
    for _ in 0..100 {
        end = o.step(50);
        if end != Slice::Waiting {
            break;
        }
        fake.0.borrow_mut().pause(40);
    }
    assert_eq!(end, Slice::Done, "the next stream read the last one's bytes");
    assert_eq!(fake.0.borrow().resets, 2, "the next open's reset reached the proxy");
}

/* The refusals sent again are the kernel's own: its share rule, its full inbox. */
#[test]
fn no_room_is_the_kernels_ebusy_and_eagain() {
    let errnos = include_str!("../../../src/syscall/microkernel/errnos.rs");
    let call = include_str!("../../../src/syscall/microkernel/ipc/call/sys_ipc_call.rs");
    let route = include_str!("../../../src/ipc/kernel_ipc.rs");
    assert!(errnos.contains("pub const ERRNO_BUSY: i64 = -16;"));
    assert!(call.contains("Err(pending_reply::Refused::Busy) => return ERRNO_BUSY,"));
    assert!(route.contains("pub const EAGAIN: i32 = -11;"));
    assert!(route.contains("Err(StrictEnqueueError::QueueFull(_)) => Err(EAGAIN),"));
    assert_eq!(NO_ROOM, [EBUSY, -11]);
}
