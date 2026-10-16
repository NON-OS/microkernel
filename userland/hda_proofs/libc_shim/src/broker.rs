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

//! The broker calls, recorded per thread in order, each with the registers a
//! test's watch reads at the moment it is made.

use std::cell::RefCell;

/// One broker call: its name, its argument, and what the watch read then.
#[derive(Clone, Debug)]
pub struct Call {
    pub name: &'static str,
    pub arg: u64,
    pub regs: Vec<u32>,
}

type Watch = Box<dyn Fn() -> Vec<u32>>;

thread_local! {
    static CALLS: RefCell<Vec<Call>> = const { RefCell::new(Vec::new()) };
    static WATCH: RefCell<Option<Watch>> = const { RefCell::new(None) };
}

/// Read the registers `f` names at every broker call from now on.
pub fn watch(f: impl Fn() -> Vec<u32> + 'static) {
    WATCH.with(|w| *w.borrow_mut() = Some(Box::new(f)));
}

/// Every broker call made on this thread, in order.
pub fn calls() -> Vec<Call> {
    CALLS.with(|c| c.borrow().clone())
}

pub(crate) fn record(name: &'static str, arg: u64) -> i64 {
    let regs = WATCH.with(|w| w.borrow().as_ref().map(|f| f()).unwrap_or_default());
    CALLS.with(|c| c.borrow_mut().push(Call { name, arg, regs }));
    0
}

pub fn mk_device_release(device_id: u64) -> i64 {
    record("device_release", device_id)
}

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    record("dma_unmap", grant_id)
}

pub fn mk_mmio_unmap(grant_id: u64) -> i64 {
    record("mmio_unmap", grant_id)
}

pub fn mk_irq_unbind(grant_id: u64) -> i64 {
    record("irq_unbind", grant_id)
}
