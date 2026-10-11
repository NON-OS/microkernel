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

//! UIC command opcodes, the result mask, and the one DME attribute this
//! driver touches (Linux ufshci.h and unipro.h).

pub const DME_GET: u32 = 0x01;
pub const DME_SET: u32 = 0x02;
pub const DME_LINKSTARTUP: u32 = 0x16;
/// MASK_UIC_COMMAND_RESULT in UCMDARG2; 0 is success.
pub const RESULT_MASK: u32 = 0xFF;

/// PA_LOCAL_TX_LCC_ENABLE. Intel's hosts take no Line Control Commands;
/// ufs_intel_disable_lcc clears it before every link startup.
pub const PA_LOCAL_TX_LCC_ENABLE: u16 = 0x155E;

/// UIC_ARG_MIB(attr): the attribute in UCMDARG1's upper half, selector 0.
pub const fn mib(attr: u16) -> u32 {
    (attr as u32) << 16
}
