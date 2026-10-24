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

//! UIC commands: link startup, failing as often as the test asks and then
//! finding the device unless it is absent; DME_GET and DME_SET on
//! PA_LOCAL_TX_LCC_ENABLE. Every command completes at once.

use super::sim::Model;
use crate::ufs::facts::hci::*;
use crate::ufs::facts::uic::*;

impl Model {
    pub(super) fn uic(&self, cmd: u32) {
        let cfg = self.cfg.borrow();
        let mut h = self.host.borrow_mut();
        let i = |r: u32| (r / 4) as usize;
        let args = [h.regs[i(UCMDARG1)], h.regs[i(UCMDARG2)], h.regs[i(UCMDARG3)]];
        h.uic.push((cmd, args));
        let attr = (args[0] >> 16) as u16;
        let (result, arg3) = match cmd {
            DME_LINKSTARTUP => {
                h.link_tries += 1;
                if h.link_tries <= cfg.link_fails {
                    (1, 0)
                } else {
                    if !cfg.no_device {
                        h.regs[i(HCS)] |= HCS_DP;
                    }
                    (0, 0)
                }
            }
            DME_GET if attr == PA_LOCAL_TX_LCC_ENABLE => (0, h.lcc),
            DME_SET if attr == PA_LOCAL_TX_LCC_ENABLE => {
                h.lcc = args[2];
                (0, args[2])
            }
            // An attribute the model does not know: INVALID_MIB_ATTRIBUTE.
            _ => (1, 0),
        };
        h.regs[i(UCMDARG2)] = result;
        h.regs[i(UCMDARG3)] = arg3;
        h.regs[i(IS)] |= IS_UCCS;
    }
}
