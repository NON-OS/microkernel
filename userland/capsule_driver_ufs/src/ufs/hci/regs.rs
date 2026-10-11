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

//! UFSHCI register offsets and bits (JESD223, as Linux ufshci.h names them).

pub const CAP: u32 = 0x00;
pub const VER: u32 = 0x08;
pub const IS: u32 = 0x20;
pub const HCS: u32 = 0x30;
pub const HCE: u32 = 0x34;
/// UIC error code, PHY adapter layer: read once after link startup to clear
/// the LINERESET the startup itself raises (ufshcd_link_startup).
pub const UECPA: u32 = 0x38;
pub const UTRLBA: u32 = 0x50;
pub const UTRLBAU: u32 = 0x54;
pub const UTRLDBR: u32 = 0x58;
pub const UTRLCLR: u32 = 0x5C;
pub const UTRLRSR: u32 = 0x60;
pub const UTMRLBA: u32 = 0x70;
pub const UTMRLBAU: u32 = 0x74;
pub const UTMRLRSR: u32 = 0x80;
pub const UICCMD: u32 = 0x90;
pub const UCMDARG1: u32 = 0x94;
pub const UCMDARG2: u32 = 0x98;
pub const UCMDARG3: u32 = 0x9C;

/// CAP: the host reaches DMA addresses above 4 GiB.
pub const CAP_64AS: u32 = 1 << 24;

/// HCS: device present, both lists ready, UIC command ready.
pub const HCS_DP: u32 = 1 << 0;
pub const HCS_UTRLRDY: u32 = 1 << 1;
pub const HCS_UTMRLRDY: u32 = 1 << 2;
pub const HCS_UCRDY: u32 = 1 << 3;
/// UFSHCD_STATUS_READY: what ufshcd_make_hba_operational waits for.
pub const HCS_LISTS_READY: u32 = HCS_UTRLRDY | HCS_UTMRLRDY | HCS_UCRDY;

pub const HCE_ENABLE: u32 = 1 << 0;
/// IS: transfer request and UIC command completion, written back to clear
/// (RW1C).
pub const IS_UTRCS: u32 = 1 << 0;
pub const IS_UCCS: u32 = 1 << 10;
pub const RUN: u32 = 1 << 0;

/// VER of a UFSHCI 1.0 or 1.1 controller, which numbers itself
/// 0x0001_0000 / 0x0001_0100 instead of major << 8 | minor << 4, and takes
/// the older command types (ufshcd_get_ufs_version).
pub const VER_1X: u32 = 1 << 16;
