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

//! The release calls the bring-up's failure path makes, each recording what
//! it was handed. A failed attempt must give back every grant and the claim,
//! or the next attempt is refused its own leftover claim.

use std::cell::RefCell;

thread_local! {
    static GIVEN_BACK: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
}

fn give_back(call: &'static str, id: u64) -> i64 {
    GIVEN_BACK.with(|g| g.borrow_mut().push((call, id)));
    0
}

/// Every release made on this thread, as (call, id), in order.
pub fn given_back() -> Vec<(&'static str, u64)> {
    GIVEN_BACK.with(|g| g.borrow().clone())
}

pub fn mk_device_release(device_id: u64) -> i64 {
    give_back("device_release", device_id)
}
pub fn mk_dma_unmap(grant: u64) -> i64 {
    give_back("dma_unmap", grant)
}
pub fn mk_irq_unbind(grant: u64) -> i64 {
    give_back("irq_unbind", grant)
}
pub fn mk_mmio_unmap(grant: u64) -> i64 {
    give_back("mmio_unmap", grant)
}

/// The uptime deadline the register waits are held to, on the host clock.
pub struct Deadline(std::time::Instant);

impl Deadline {
    pub fn after_ms(ms: u64) -> Self {
        Deadline(std::time::Instant::now() + std::time::Duration::from_millis(ms))
    }
    pub fn expired(&self) -> bool {
        std::time::Instant::now() >= self.0
    }
}
