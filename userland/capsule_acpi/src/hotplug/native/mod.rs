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

//! Native PCI Express hot plug, for the ports whose root bridge's _OSC
//! gave it to NONOS. That happens only when `boot::osc_host`'s one
//! switch asks for it, and the switch is off: until the owner turns it
//! on, firmware keeps hot plug and nothing here runs. Each slot is
//! polled; an arrival is probed into the kernel's table
//! (MkPciProbeFunction), and a departure or eject goes through
//! MkPciDepart, so the holder is told (DGON) and its windows fenced.

mod act;
mod fault;
mod find;
mod functions;
// The kernel's one definition of an _OSC grant, by path; only the grant
// test and the bits are read here.
#[allow(dead_code)]
#[path = "../../../../../src/hardware/broker/acpi/osc_grant.rs"]
pub mod osc_grant;
mod poll;
mod power;
mod read;
pub mod regs;
pub mod slot;
mod step;
mod step_timed;
pub mod times;
mod walk;

pub use find::find;
pub use poll::tick;

/// The hot-plug ports and when they are next read.
#[derive(Default)]
pub struct Native {
    pub ports: alloc::vec::Vec<find::Port>,
    pub next_ms: u64,
}
