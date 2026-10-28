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

//! The USB side (wIndex 0000h).

pub const USB2_PHY: u16 = 0xb41e;
pub const SS_LINK1: u16 = 0xb426;
pub const SS_LINK2: u16 = 0xb428;
pub const U2P3: u16 = 0xb460;
pub const SPARE2: u16 = 0xb466;
pub const CONNECT_TIMER: u16 = 0xcbf8;
pub const MSC_TIMER: u16 = 0xcbfc;
pub const LPM_CONFIG: u16 = 0xcfd8;
pub const MISC2: u16 = 0xcfff;
pub const USB_CTRL: u16 = 0xd406;
pub const RX_THRESHOLD: u16 = 0xd40c;
pub const LPM_CTRL: u16 = 0xd41a;
pub const RX_EARLY_TIMEOUT: u16 = 0xd42c;
pub const RX_EARLY_SIZE: u16 = 0xd42e;
pub const RX_EXTRA_AGG: u16 = 0xd432;
pub const RX_DMA_OWN: u16 = 0xd437;
/// The U1 and U2 link states the device takes: six bytes from here.
pub const U1U2: u16 = 0xd490;
pub const BMU_RESET: u16 = 0xd4b0;
pub const FW_TASK: u16 = 0xd4e8;
pub const POWER_CUT: u16 = 0xd80a;
pub const MISC0: u16 = 0xd81a;
pub const AFE2: u16 = 0xd824;
pub const WATCHDOG11: u16 = 0xe43c;
