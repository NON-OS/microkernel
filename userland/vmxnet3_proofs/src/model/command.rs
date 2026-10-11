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

//! Commands, and the answer each leaves in CMD.

use super::device::Device;
use super::events::Event;

impl Device {
    pub(super) fn command(&mut self, cmd: u32) {
        let grants_back = nonos_libc::given_back().len();
        self.note(Event::Command { cmd, grants_back });
        self.answer = match cmd {
            0xCAFE_0000 => match self.activate() {
                Ok(()) => {
                    self.active = true;
                    0
                }
                Err(why) => {
                    self.refusal = Some(why);
                    1
                }
            },
            0xCAFE_0001 | 0xCAFE_0002 => {
                self.active = false;
                0
            }
            0xCAFE_0003 => {
                self.rx_mode = self.mem.u32(self.dsa + 120);
                0
            }
            0xCAFE_0004 => 0,
            0xCAFE_0005 => {
                self.vf0 = self.mem.u32(self.dsa + 136);
                0
            }
            0xF00D_0000 => {
                let qd = self.mem.u64(self.dsa + 40);
                let stopped = self.faults.stop_queues as u8;
                self.mem.set_u8(qd + 80, stopped);
                self.mem.set_u8(qd + 256 + 80, stopped);
                0
            }
            0xF00D_0002 => (10000 << 16) | self.link as u32,
            _ => panic!("unknown command {cmd:#x}"),
        };
    }
}
