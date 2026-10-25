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
//! Each test thread is a server of its own (server/served.rs): a reply one
//! thread holds for the multiplexer never holds another thread's, and a
//! sender or a vouch set on one is not seen on another. Shared, a test
//! whose multiplexer held a reply swallowed the reply of a test beside it,
//! and its release let a second reply through (the revive tests' flake).

use std::sync::{Arc, Barrier};
use std::thread;

use nonos_libc::take_replies;

use crate::server::reply::{captured, send, sender, set_sender, vouch, vouched};

#[test]
fn a_reply_held_on_one_thread_holds_no_other_threads_reply() {
    let (inside, done) = (Arc::new(Barrier::new(2)), Arc::new(Barrier::new(2)));
    let (i, d) = (inside.clone(), done.clone());
    let holder = thread::spawn(move || {
        let held = captured(|| {
            i.wait();
            d.wait();
            send([7u8; 4].as_ptr(), 4);
        });
        (held, take_replies().len(), sender(), vouched())
    });
    inside.wait();
    set_sender(9);
    vouch();
    send([1u8; 8].as_ptr(), 8);
    assert_eq!(take_replies().len(), 1, "sent while the other thread holds its own");
    done.wait();
    let (held, sent, its_sender, its_vouch) = holder.join().expect("the holder");
    assert_eq!((held, sent), (4, 0), "the holder's own reply held, and only that");
    assert_eq!((its_sender, its_vouch), (0, false), "this thread's sender and vouch");
    assert_eq!((sender(), vouched()), (9, true));
}
