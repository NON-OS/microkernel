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

use super::init_shared::{init_shared, wait_ll_share_fifo_ready};
use super::ocp::mac_ocp_modify;
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Linux rtl_hw_init_8168g (VER_40 to VER_52): the shared start, then MAC
/// OCP 0xE8DE bit 15. The 8168ep and 8117 (VER_51, VER_52) also stop their
/// DASH CMAC first in Linux; that is not done here.
pub fn init_8168g(regs: &Regs, ver: MacVersion) {
    init_shared(regs, ver);
    mac_ocp_modify(regs, 0xE8DE, 0, 1 << 15);
    wait_ll_share_fifo_ready(regs);
}
