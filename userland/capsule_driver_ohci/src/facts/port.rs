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

//! The root hub: HcRhDescriptorA, HcRhStatus and one HcRhPortStatus per
//! port. A port register reads as status and is written as commands, each
//! bit its own; a change bit clears when written as one.

pub const HC_RH_DESCRIPTOR_A: u32 = 0x48;
pub const HC_RH_STATUS: u32 = 0x50;
pub const HC_RH_PORT_STATUS: u32 = 0x54;
/// At most 15 downstream ports (NDP).
pub const PORTS_MAX: u8 = 15;

/// HcRhDescriptorA: the port count, no power switching, and the power-on
/// to power-good time in 2 ms units.
pub const RHA_NDP: u32 = 0xFF;
pub const RHA_NPS: u32 = 1 << 9;
pub const RHA_POTPGT_SHIFT: u32 = 24;

/// HcRhStatus written: set global power.
pub const RHS_LPSC: u32 = 1 << 16;

/// Read: connected, enabled, low speed; connect, enable and reset-done
/// changes.
pub const PORT_CCS: u32 = 1 << 0;
pub const PORT_PES: u32 = 1 << 1;
pub const PORT_LSDA: u32 = 1 << 9;
pub const PORT_CSC: u32 = 1 << 16;
pub const PORT_PESC: u32 = 1 << 17;
pub const PORT_PRSC: u32 = 1 << 20;

/// Written: clear enable, set reset, set power.
pub const PORT_CLEAR_ENABLE: u32 = 1 << 0;
pub const PORT_SET_RESET: u32 = 1 << 4;
pub const PORT_SET_POWER: u32 = 1 << 8;
