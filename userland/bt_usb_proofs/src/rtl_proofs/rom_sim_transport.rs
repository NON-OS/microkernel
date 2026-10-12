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

//! The model ROM behind the transport the host drives: each command is
//! answered at once, so the host never waits.

use super::rom_sim::RomSim;
use crate::hci_sim::events::complete;
use nonos_bt_hci::{HciError, Transport};

impl Transport for RomSim {
    fn send_command(&mut self, p: &[u8]) -> Result<(), HciError> {
        let (op, params) = (u16::from_le_bytes([p[0], p[1]]), &p[3..]);
        self.ops.push(op);
        let e = match op {
            _ if self.refuse == Some(op) => complete(1, op, 0x12, &[]),
            0x1001 => complete(1, op, 0, &self.version()),
            0xfc6d => complete(1, op, 0, &[self.rom]),
            0xfc61 => {
                self.asked_register = Some(params.to_vec());
                complete(1, op, 0, &self.register)
            }
            0xfc20 => {
                self.frags.push((params[0], params[1..].to_vec()));
                complete(1, op, 0, if self.terse { &[] } else { &params[..1] })
            }
            _ => complete(1, op, 0x01, &[]),
        };
        self.events.push_back(e);
        Ok(())
    }
    fn send_acl(&mut self, _: &[u8]) -> Result<(), HciError> {
        Ok(())
    }
    fn poll_event(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        Ok(self.events.pop_front())
    }
    fn poll_acl(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        Ok(None)
    }
    fn pause(&mut self) {}
}

impl RomSim {
    pub(super) fn version(&self) -> Vec<u8> {
        let ended = self.frags.last().is_some_and(|(i, _)| i & 0x80 != 0);
        let sub =
            if ended { self.patched.unwrap_or(self.lmp_subversion) } else { self.lmp_subversion };
        let [r0, r1] = self.hci_revision.to_le_bytes();
        let ([c0, c1], [s0, s1]) = (self.company.to_le_bytes(), sub.to_le_bytes());
        vec![self.hci_version, r0, r1, 8, c0, c1, s0, s1]
    }
}
