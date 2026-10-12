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

//! UIC commands and the one DME attribute the engine touches. Source:
//! JESD223 (UIC command opcodes, result in UCMDARG2) and MIPI UniPro (the
//! attribute id), as recalled, unverified.

pub const DME_GET: u32 = 0x01;
pub const DME_SET: u32 = 0x02;
pub const DME_LINKSTARTUP: u32 = 0x16;
/// The result field of UCMDARG2; 0 is success.
pub const RESULT_MASK: u32 = 0xFF;

/// PA_Local_TX_LCC_Enable. Intel's hosts want Line Control Commands off
/// before link startup: observed in Linux ufshcd-pci.c, unverified against
/// Intel's documents.
pub const PA_LOCAL_TX_LCC_ENABLE: u16 = 0x155E;

/// A local attribute in UCMDARG1: its id in the upper half, selector 0.
pub const fn mib(attr: u16) -> u32 {
    (attr as u32) << 16
}
