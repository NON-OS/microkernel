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

//! A controller that restarts just after the host's packet with a chosen
//! count: out loud (Hardware Error, the link forgotten), silently, or by
//! saying the link's encryption is off. Every ACL from the host that
//! reaches the link once the host has read that is counted: none may.

use super::state::{Air, HANDLE};
use crate::hci_sim::link_events::encryption_change;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Restart {
    Loud,
    Silent,
    Unsealed,
}

impl Air {
    fn step(&mut self) {
        self.lies.steps += 1;
        let Some((_, how)) = self.lies.restart.filter(|r| r.0 == self.lies.steps) else { return };
        match how {
            Restart::Loud => {
                (self.lies.forgot, self.lies.sealed.0) = (true, false);
                self.event(&[0x10, 1, 0x00]);
            }
            Restart::Silent => (self.lies.forgot, self.lies.sealed.0) = (true, false),
            Restart::Unsealed => {
                self.event(&encryption_change(0, HANDLE, 0));
                self.lies.unsealing = Some(self.interrupt.len());
            }
        }
    }

    /// One interrupt transfer taken: past the encryption-off event, the host knows.
    pub(super) fn read_one(&mut self) {
        if let Some(n) = self.lies.unsealing.as_mut() {
            *n -= 1;
            if *n == 0 {
                (self.lies.unsealing, self.lies.sealed.0) = (None, false);
            }
        }
    }

    /// A command from the host.
    pub(super) fn took(&mut self, packet: &[u8]) {
        self.commands.push(packet.to_vec());
        self.step();
    }

    /// An ACL packet from the host; whether it reaches no link at all.
    pub(super) fn took_acl(&mut self) -> bool {
        let (now, ever) = self.lies.sealed;
        if ever && !now && !self.lies.forgot {
            self.lies.clear += 1;
        }
        let gone = self.lies.forgot;
        self.step();
        gone
    }
}
