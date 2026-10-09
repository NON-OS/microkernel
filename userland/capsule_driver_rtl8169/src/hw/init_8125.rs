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
use super::ocp::mac_ocp_write;
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Linux rtl_hw_init_8125 (VER_61 on): the shared start, then three MAC
/// OCP words Realtek gives without a name, and the link list again.
pub fn init_8125(regs: &Regs, ver: MacVersion) {
    init_shared(regs, ver);
    mac_ocp_write(regs, 0xC0AA, 0x07D0);
    mac_ocp_write(regs, 0xC0A6, 0x0150);
    mac_ocp_write(regs, 0xC01E, 0x5555);
    wait_ll_share_fifo_ready(regs);
}
