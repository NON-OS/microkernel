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
//! The link as net_core drives it, serviced between its requests.

use super::super::super::region::{Clock, Region};
use super::super::fw::{Fw, Queue};
use super::state::Link;
use crate::regs::Mmio;
/// The link as net_core drives it: the firmware and the open port.
pub struct Port<'p, 'a, 'd, M: Mmio, R: Region + ?Sized, C: Clock> {
    pub fw: &'p mut Fw<'a, 'd, M, R, C>,
    pub link: &'p mut Link,
}

impl<M: Mmio, R: Region + ?Sized, C: Clock> Port<'_, '_, '_, M, R, C> {
    /// Take what the firmware posted and answer what calls for it.
    pub fn service(&mut self) {
        self.fw.pump();
        while let Some(rx) = self.fw.inbox.frames.pop_front() {
            if let Some(reply) = self.link.receive(rx) {
                let rate = self.link.mgmt_rate;
                let _ = self.fw.send(Queue::Data, &reply, rate);
            }
        }
    }
}
