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

//! A kernel that never has room for a caller's calls: its reset is sent
//! again until the open's bound, and the proxy is then said to be silent,
//! with nothing having reached it.

use std::cell::Cell;
use std::rc::Rc;

use crate::bounds::OPEN_MS;
use crate::carrier::Carrier;
use crate::crowded::Crowded;
use crate::fake::{FakeProxy, Shared};
use crate::opening::Opening;
use crate::refusal::Proxy;
use crate::slice::Slice;
use crate::tunnel::Tunnel;

#[test]
fn an_open_the_kernel_never_has_room_for_is_silent_within_its_bound() {
    let fake = Shared::new(FakeProxy::nym());
    let crowd = Rc::new(Cell::new(usize::MAX));
    let start = fake.0.borrow().now;
    let why = Tunnel::open(Crowded(fake.clone(), crowd.clone()), Proxy::Nym, "h", 1).err();
    assert_eq!(why, Some(Proxy::Nym.silent()));
    let spent = fake.0.borrow().now - start;
    assert!(spent <= OPEN_MS + 100, "the blocking open kept its bound: {spent}");

    let Ok(mut o) = Opening::begin(Crowded(fake.clone(), crowd), Proxy::Anyone, "h", 1, 50) else {
        panic!("did not begin")
    };
    let start = fake.0.borrow().now;
    let mut end = Slice::Waiting;
    while end == Slice::Waiting && fake.0.borrow().now - start <= 2 * OPEN_MS {
        end = o.step(50);
        fake.0.borrow_mut().pause(40);
    }
    assert_eq!(end, Slice::Failed(Proxy::Anyone.silent()));
    let spent = fake.0.borrow().now - start;
    assert!(spent <= OPEN_MS + 300, "the sliced open kept its bound: {spent}");
    assert_eq!(fake.0.borrow().calls, 0, "nothing reached the proxy");
}
