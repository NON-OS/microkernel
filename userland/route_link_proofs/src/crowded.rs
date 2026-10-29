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

//! A kernel with no room for a caller's calls. It holds each caller to
//! eight unanswered calls at a proxy (pending_reply/share.rs) and refuses
//! the next outright, so the proxy never sees it. The Linux personality is
//! one caller for every guest stream, and net.socks5 answers nothing while
//! it sits in a mixnet send, so its share fills within a second or two.

use std::cell::Cell;
use std::rc::Rc;

use crate::carrier::Carrier;
use crate::fake::{FakeProxy, Shared};
use crate::refusal::Proxy;
use crate::tunnel::Tunnel;

/* The kernel's EBUSY: the caller already holds its share of the places. */
pub const EBUSY: i64 = -16;

/* The fake proxy behind a kernel that refuses the next `.1` calls. */
pub struct Crowded(pub Shared, pub Rc<Cell<usize>>);

impl Carrier for Crowded {
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64 {
        match self.1.replace(self.1.get().saturating_sub(1)) {
            0 => self.0.call(frame, wait_ms, into),
            _ => EBUSY,
        }
    }

    fn now_ms(&self) -> i64 {
        self.0.now_ms()
    }

    fn pause(&mut self, ms: u64) {
        self.0.pause(ms)
    }
}

/*
 * A stream on which a TLS hello was written and let go unread while the
 * node answered it, as monero-wallet-cli's first try at TLS is, with its
 * reset refused and the next open's first one refused too. The proxy, and
 * the carrier the next open on the same stream goes through.
 */
pub fn after_last() -> (Shared, Crowded) {
    let fake = Shared::new(FakeProxy::nym().deliver(0, b"HTTP/1.1 400 Bad Request\r\n"));
    let crowd = Rc::new(Cell::new(0));
    let opened = Tunnel::open(Crowded(fake.clone(), crowd.clone()), Proxy::Nym, "n.example", 1);
    let Ok(mut last) = opened else { panic!("the last stream did not open") };
    assert_eq!(last.write_all(b"\x16\x03\x01\x02\x00"), Ok(()));
    crowd.set(2);
    drop(last);
    (fake.clone(), Crowded(fake, crowd))
}
