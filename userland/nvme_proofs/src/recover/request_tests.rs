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

//! A request on the driver's queue, as the server serves it: a failure
//! worth it is sent again, the reply carries the controller's own status,
//! and a controller gone or a reset failed takes the queue with it, so
//! every later request is refused without touching the device.

use nonos_libc::logged;

use super::server::Server;
use crate::fake::{register, up};
use crate::nvm::sq_tail_doorbell;
use crate::protocol::{device_status, E_NODEV, E_TIMEDOUT};

#[test]
fn a_transient_failure_is_sent_again_and_the_reply_is_ok() {
    let mut s = Server::new(up());
    s.fake.borrow_mut().fail = vec![(1, 0x06 << 1)]; // Internal Error
    let r = s.read(None);
    assert_eq!((r.status, r.sent), (Ok(()), 2));
}

#[test]
fn a_failure_not_worth_retrying_answers_with_the_controllers_status() {
    let mut s = Server::new(up());
    // Unrecovered Read Error (SCT 2, SC 81h), then Invalid Field with DNR.
    s.fake.borrow_mut().fail = vec![(1, (2 << 9) | (0x81 << 1)), (2, 0x8000 | (0x02 << 1))];
    let (media, refused) = (s.read(None), s.read(None));
    assert_eq!((media.status, media.sent), (Err(device_status(2, 0x81)), 1));
    assert_eq!((refused.status, refused.sent), (Err(device_status(0, 0x02)), 2));
    assert!(s.io.as_ref().is_some_and(|io| io.stuck().is_none()), "nothing left out");
}

#[test]
fn a_controller_gone_or_a_reset_failed_takes_the_queue_with_it() {
    for (gone, words) in [(true, "gone from the bus"), (false, "reset failed")] {
        let mut s = Server::new(up());
        s.fake.borrow_mut().stall.push(1);
        (s.fake.borrow_mut().gone, s.fake.borrow_mut().wedged) = (gone, !gone);
        let timed_out = s.read(None);
        assert_eq!(timed_out.status, Err(E_TIMEDOUT), "the timed-out request");
        assert!(s.io.is_none(), "{words}: the queue is dropped");
        assert!(logged().last().is_some_and(|l| l.contains(words)), "{:?}", logged().last());
        let from = s.fake.borrow().admin_seen.len();
        let sent = register(s.device.regs, sq_tail_doorbell(1, 0));
        let next = s.read(None);
        assert_eq!((next.status, next.ms), (Err(E_NODEV), 0), "refused at once");
        assert_eq!((s.ops_since(from).len(), next.sent), (0, sent), "the device left alone");
    }
}
