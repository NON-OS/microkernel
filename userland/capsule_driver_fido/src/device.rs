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
//! What the driver holds: the key bound, if any, who holds it, the request
//! in flight, when the key last spoke, and whether the channel is spent.

use nonos_libc::Deadline;
use nonos_usbnet::scan::Bound;
use nonos_usbnet::xhci::XhciBus;

use crate::bind::Port;
use crate::ctaphid::Job;
use crate::owner::Owner;

/// How long a running request may go without word from the key; keys
/// send a keep-alive several times a second while they wait for a touch.
pub const QUIET_MS: u64 = 5_000;

pub struct Dev {
    pub bound: Option<Bound<Port>>,
    pub owner: Owner,
    pub job: Job,
    pub quiet: Deadline,
    /// A request ended without its answer: a new channel before the next.
    pub spent: bool,
}

impl Default for Dev {
    fn default() -> Self {
        let quiet = Deadline::after_ms(0);
        Self { bound: None, owner: Owner::default(), job: Job::Idle, quiet, spent: false }
    }
}

impl Dev {
    pub fn bus(&self) -> Option<XhciBus> {
        self.bound.as_ref().map(|b| XhciBus { xhci: b.xhci, slot: b.slot })
    }

    pub fn port(&self) -> Option<&Port> {
        self.bound.as_ref().map(|b| &b.nic)
    }
}
