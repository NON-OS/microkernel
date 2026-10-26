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

//! Looking for the device and keeping it, one bounded step at a time between
//! requests (server/serve.rs): the probe that ran inline held every request
//! about thirty-five seconds, every few seconds, for an empty card reader.

use super::asking::Asking;
use super::identity::Identity;
use super::medium::Medium;
use super::ports::Ports;
use super::report::Report;
use super::searching::XHCI_WAIT_MS;
use crate::clock::Deadline;
use crate::xhci::{hosts, Hosts};

pub struct Scanner {
    /// Every USB host, fastest first, each with what its root ports showed;
    /// how many have registered.
    pub(super) hosts: Hosts<Ports>,
    pub(super) known: usize,
    /// While the search lasts a block request is asked to come again.
    pub(super) searching: bool,
    pub(super) window: Deadline,
    pub(super) settle: Deadline,
    /// When the ports are next listed; whether the last listing left any open.
    pub(super) look: Deadline,
    pub(super) open: bool,
    pub(super) asking: Option<Asking>,
    /// The device served this boot, once there is one (`identity`); the
    /// host and port it was let go on while it stays there (`release`).
    pub(super) served: Option<Identity>,
    pub(super) left_on: Option<(u32, u8)>,
    /// Devices served after the first, and media served afresh (`asked`),
    /// which the kernel reads.
    pub rebinds: u32,
    pub medium: Medium,
    /// What the search last found, which the kernel asks for (OP_GET_STATE).
    pub report: Report,
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        let wait = Deadline::after_ms(XHCI_WAIT_MS);
        Self {
            hosts: hosts(Ports::new),
            known: 0,
            searching: true,
            window: wait,
            settle: wait,
            look: Deadline::after_ms(0),
            open: false,
            asking: None,
            served: None,
            left_on: None,
            rebinds: 0,
            medium: Medium::Scanning,
            report: Report::new(),
        }
    }
}
