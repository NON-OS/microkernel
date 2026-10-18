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

//! The release calls, recorded per thread: a failed bring-up attempt must
//! give back every grant and the claim, or the next attempt finds the device
//! still held. Each is logged with what the part was doing (`steps`).

use std::cell::RefCell;

use crate::pci::stop_bus_master;
use crate::steps::{note, Step};

/// The port grant as `mk_pio_grant` reports it; a rollback reaches the part
/// through it.
pub struct PioGrantOut {
    pub port_base: u16,
    pub port_count: u16,
    pub _pad: u32,
    pub grant_id: u64,
}

thread_local! {
    static GIVEN_BACK: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
}

fn give_back(call: &'static str, id: u64) -> i64 {
    note(Step::Freed(call, id));
    GIVEN_BACK.with(|g| g.borrow_mut().push((call, id)));
    0
}

/// Every release the driver made on this thread, as (call, id), in order.
pub fn given_back() -> Vec<(&'static str, u64)> {
    GIVEN_BACK.with(|g| g.borrow().clone())
}

/// Logged with Bus Master as the driver left it; the release then clears it
/// before any grant goes, as `MkDeviceRelease` does.
pub fn mk_device_release(device_id: u64) -> i64 {
    let r = give_back("device_release", device_id);
    stop_bus_master();
    r
}
pub fn mk_dma_unmap(grant: u64) -> i64 {
    give_back("dma_unmap", grant)
}
pub fn mk_pio_release(grant: u64) -> i64 {
    give_back("pio_release", grant)
}
