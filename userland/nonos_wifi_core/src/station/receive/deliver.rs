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

//! A checked frame goes on: EAPOL to the supplicant, the rest to the stack.

use super::*;

impl LinkStation {
    pub(super) fn deliver(&mut self, c: &Checked) -> Rx {
        let body = &c.body;
        if body.len() < 8 || body[..6] != LLC_SNAP {
            return Rx::Dropped(RxDrop::NotLlc);
        }
        if body[6..8] == ETHERTYPE_EAPOL {
            return self.on_eapol(&body[8..], c.protected);
        }
        let mut eth = Vec::with_capacity(14 + body.len() - 8);
        eth.extend_from_slice(&c.da);
        eth.extend_from_slice(&c.sa);
        eth.extend_from_slice(&body[6..]);
        Rx::Ethernet(eth)
    }
}
