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

//! What the modelled controller does with each command. Intel Reset into
//! the firmware takes a soft reset with patches on, DDC not reloaded and
//! boot option 1, and is answered by the bootup event alone; anything it
//! does not know is answered Unknown HCI Command (0x01).

use super::model::Intel;
use nonos_bt_wire::hci::command::parse_command;

impl Intel {
    pub(super) fn command(&mut self, packet: &[u8]) {
        let (op, p) = parse_command(packet).expect("a whole command");
        self.ops.push(op.0);
        let booted = usize::from(self.booted);
        let ret = match (op.0, p) {
            (0xfc05, [0xff]) if self.in_iml => self.iml.clone().expect("a loader").0,
            (0xfc05, [0xff]) => self.tlv[booted].clone(),
            (0xfc05, []) => self.old[booted].clone(),
            (0xfc0d, []) if !self.booted => self.boot_params.clone(),
            (0xfc01, &[0, 1, 0, 1, a, b, c, d]) if !self.booted => {
                self.boot_addr = Some(u32::from_le_bytes([a, b, c, d]));
                self.boot_addrs.push(u32::from_le_bytes([a, b, c, d]));
                match &self.iml {
                    Some((_, next)) if !self.in_iml => {
                        (self.in_iml, self.data, self.expect) = (true, 0, *next)
                    }
                    _ => (self.in_iml, self.booted) = (false, self.boots),
                }
                if self.boots {
                    self.event(&[0xff, 7, 0x02, 0, 1, 0, 0, 0, 0]);
                }
                return;
            }
            (0xfc01, [1, 1, 1, 0, 0, 0, 0, 0]) => return self.to_bootloader += 1,
            (0xfc8b, entry) if self.booted => {
                self.ddc.push(entry.to_vec());
                Vec::new()
            }
            (0xfc0a, [0, 0]) if self.in_iml => Vec::new(),
            (0xfc52, [0x87, 0x0c, 0, 0, 0, 0, 0, 0]) | (0x0c03, []) => Vec::new(),
            _ => return self.complete(op.0, 0x01, &[]),
        };
        self.complete(op.0, 0, &ret);
    }
}
