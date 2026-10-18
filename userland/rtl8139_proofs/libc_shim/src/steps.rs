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

//! A log of every PCI Command write and every grant given back, each with
//! Bus Master as it stood just before and what the part was doing at that
//! instant, as the test's probe reads it. The order these hold the driver to
//! is the one teardown and bring-up are written for: no frame goes back
//! while an engine runs or Bus Master is on, and Bus Master goes on only
//! after the reset and the rings.

use std::cell::RefCell;

/// The part at one instant: resets it has completed, its engines, and
/// whether both rings name this driver's own frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Part {
    pub resets: u32,
    pub rx_on: bool,
    pub tx_on: bool,
    pub rings_own: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Command(u16),
    Freed(&'static str, u64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seen {
    pub step: Step,
    pub bus_master: bool,
    pub part: Part,
}

type Probe = Box<dyn Fn() -> Part>;

thread_local! {
    static SEEN: RefCell<Vec<Seen>> = const { RefCell::new(Vec::new()) };
    static PROBE: RefCell<Option<Probe>> = const { RefCell::new(None) };
}

/// Start a new log, with `probe` reading the part.
pub fn watch(probe: impl Fn() -> Part + 'static) {
    SEEN.with(|s| s.borrow_mut().clear());
    PROBE.with(|p| *p.borrow_mut() = Some(Box::new(probe)));
}

/// Every step logged since `watch`, in order.
pub fn seen() -> Vec<Seen> {
    SEEN.with(|s| s.borrow().clone())
}

pub(crate) fn note(step: Step) {
    let part = PROBE.with(|p| p.borrow().as_ref().map(|f| f()).unwrap_or_default());
    let bus_master = crate::pci::bus_master_on();
    SEEN.with(|s| s.borrow_mut().push(Seen { step, bus_master, part }));
}
