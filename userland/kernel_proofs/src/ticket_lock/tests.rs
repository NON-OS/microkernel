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

use super::take;
use super::ticket::TicketMutex;

#[test]
fn numbers_are_served_in_the_order_drawn() {
    let lock = TicketMutex::new(0u64);
    let (first, second, third) = (lock.draw(), lock.draw(), lock.draw());
    assert!(lock.served(second).is_none() && lock.served(third).is_none());
    let held = lock.served(first).expect("the first number is served at once");
    assert!(lock.served(second).is_none(), "served while the first is held");
    drop(held);
    assert!(lock.served(third).is_none(), "the third came before the second");
    drop(lock.served(second).expect("the second is next"));
    assert!(lock.served(third).is_some());
}

#[test]
fn a_waiter_comes_before_a_holder_taking_again() {
    // The regression's shape: one CPU lets go and takes again at once, the
    // other has been waiting. The waiter's number is the earlier one.
    let lock = TicketMutex::new(0u64);
    let hog = take(&lock);
    let waiter = lock.draw();
    drop(hog);
    let again = lock.draw();
    assert!(lock.served(again).is_none(), "the hog took it back first");
    drop(lock.served(waiter).expect("the waiter is served"));
    assert!(lock.served(again).is_some());
}

#[test]
fn try_lock_takes_only_a_free_lock_with_nobody_waiting() {
    let lock = TicketMutex::new(0u64);
    let held = lock.try_lock().expect("a free lock is taken");
    assert!(lock.try_lock().is_none(), "taken twice");
    let waiter = lock.draw();
    drop(held);
    assert!(lock.try_lock().is_none(), "jumped the queue");
    drop(lock.served(waiter).unwrap());
    assert!(lock.try_lock().is_some());
}

#[test]
fn a_try_that_fails_draws_nothing() {
    let lock = TicketMutex::new(0u64);
    let held = take(&lock);
    for _ in 0..1000 {
        assert!(lock.try_lock().is_none());
    }
    drop(held);
    // A failed try left no number behind to stop the line.
    drop(take(&lock));
}
