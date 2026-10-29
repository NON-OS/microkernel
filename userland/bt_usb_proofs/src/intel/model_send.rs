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

//! What the modelled bootloader does with a Secure Send: each fragment is
//! answered, and once the last data octet has come, the result is told,
//! before or after that fragment's Command Complete.

use super::model::Intel;
use nonos_bt_wire::hci::command::parse_command;

impl Intel {
    pub(super) fn secure_send(&mut self, packet: &[u8]) {
        let (op, p) = parse_command(packet).expect("a whole command");
        assert_eq!(op.0, 0xfc09, "only Secure Send goes on bulk OUT");
        assert!(!self.booted, "Secure Send to booted firmware");
        assert!((2..=253).contains(&p.len()), "a fragment of 1 to 252 octets");
        self.ops.push(op.0);
        self.fragments.push(p.to_vec());
        self.data += if p[0] == 0x01 { p.len() - 1 } else { 0 };
        let done = p[0] == 0x01 && self.data == self.expect;
        let refused = self.fragments.len() == self.refuse_fragment;
        let result = [0xff, 5, 0x06, self.result, 0x09, 0xfc, 0x00];
        if done && self.result_first {
            self.event(&result);
        }
        self.complete(0xfc09, if refused { 0x12 } else { 0 }, &[]);
        if done && !self.result_first {
            self.event(&result);
        }
    }
}
