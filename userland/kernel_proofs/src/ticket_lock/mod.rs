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

/*
 * The paging manager's lock: taken in the order it was asked for, so a CPU
 * that lets go and takes again in a loop cannot keep another out
 * (regression-30a4b58c4-paging-lock). The kernel's own two files.
 */

#[cfg(test)]
mod race_tests;
#[cfg(test)]
mod tests;
#[path = "../../../../src/smp/ticket.rs"]
pub mod ticket;
#[path = "../../../../src/smp/ticket_guard.rs"]
pub mod ticket_guard;

/// The kernel's take (smp/responsive_lock.rs) less the shootdown service:
/// draw a number, then wait for it. A host thread is not pinned to a CPU,
/// so the wait yields: a number's owner descheduled would stall the rest.
#[cfg(test)]
fn take<T>(lock: &ticket::TicketMutex<T>) -> ticket::TicketGuard<'_, T> {
    wait(lock, lock.draw())
}

#[cfg(test)]
fn wait<T>(lock: &ticket::TicketMutex<T>, number: u32) -> ticket::TicketGuard<'_, T> {
    loop {
        if let Some(guard) = lock.served(number) {
            return guard;
        }
        std::thread::yield_now();
    }
}
