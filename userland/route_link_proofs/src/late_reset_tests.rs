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

//! A reset the kernel queued is taken before the greeting whenever its
//! answer comes, so only one it had no room for is sent again: an answer
//! that came too late does not hold the greeting back.

use crate::carrier::Carrier;
use crate::fake::{FakeProxy, Shared};
use crate::opening::Opening;
use crate::refusal::Proxy;
use crate::slice::Slice;
use crate::tunnel::Tunnel;

/* The kernel's ETIMEDOUT: the call was queued, its answer came too late. */
const ETIMEDOUT: i64 = -110;

/* Resets the proxy takes, each answered after its caller stopped waiting. */
struct LateResets(Shared);

impl Carrier for LateResets {
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64 {
        let got = self.0.call(frame, wait_ms, into);
        match frame {
            [1] => ETIMEDOUT,
            _ => got,
        }
    }

    fn now_ms(&self) -> i64 {
        self.0.now_ms()
    }

    fn pause(&mut self, ms: u64) {
        self.0.pause(ms)
    }
}

#[test]
fn a_reset_queued_but_answered_late_lets_the_greeting_go_at_once() {
    let fake = Shared::new(FakeProxy::nym());
    let Ok(mut o) = Opening::begin(LateResets(fake.clone()), Proxy::Nym, "h", 1, 50) else {
        panic!("did not begin")
    };
    let ends: Vec<Slice> = (0..5).map(|_| o.step(50)).collect();
    assert_eq!(ends[4], Slice::Done, "reset, greeting, method, CONNECT, reply: {ends:?}");
    assert_eq!(fake.0.borrow().resets, 1);
    drop(o);
    let blocking = Tunnel::open(LateResets(fake.clone()), Proxy::Nym, "h", 1);
    assert!(blocking.is_ok_and(|t| t.pending.is_empty()));
    assert_eq!(fake.0.borrow().resets, 4, "one in, one out for each open");
}
