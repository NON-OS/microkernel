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

//! The capability registers (from BAR0) and the operational registers
//! (from BAR0 plus CAPLENGTH), and the bits this driver uses.

/// Capability registers: CAPLENGTH (a byte), HCSPARAMS, HCCPARAMS.
pub const CAPLENGTH: u32 = 0x00;
pub const HCSPARAMS: u32 = 0x04;
pub const HCCPARAMS: u32 = 0x08;
/// HCSPARAMS: the port count, and whether ports have power switches.
pub const HCS_N_PORTS: u32 = 0xF;
pub const HCS_PPC: u32 = 1 << 4;
/// HCCPARAMS: 64-bit addressing, and the legacy capability's config
/// offset (EECP).
pub const HCC_64BIT: u32 = 1 << 0;
pub const HCC_EECP_SHIFT: u32 = 8;

/// Operational registers, offsets from the operational base.
pub const USBCMD: u32 = 0x00;
pub const USBSTS: u32 = 0x04;
pub const USBINTR: u32 = 0x08;
pub const FRINDEX: u32 = 0x0C;
pub const CTRLDSSEGMENT: u32 = 0x10;
pub const PERIODICLISTBASE: u32 = 0x14;
pub const ASYNCLISTADDR: u32 = 0x18;
pub const CONFIGFLAG: u32 = 0x40;

/// USBCMD: run, host reset, the periodic and async schedules on, the
/// async advance doorbell, and an interrupt threshold of 8 micro-frames.
pub const CMD_RS: u32 = 1 << 0;
pub const CMD_HCRESET: u32 = 1 << 1;
pub const CMD_PSE: u32 = 1 << 4;
pub const CMD_ASE: u32 = 1 << 5;
pub const CMD_IAAD: u32 = 1 << 6;
pub const CMD_ITC_8: u32 = 8 << 16;

/// USBSTS: async advance, halted, and the schedules' status; every
/// interrupt status bit (each cleared by a one).
pub const STS_IAA: u32 = 1 << 5;
pub const STS_HALTED: u32 = 1 << 12;
pub const STS_PSS: u32 = 1 << 14;
pub const STS_ASS: u32 = 1 << 15;
pub const STS_ALL: u32 = 0x3F;

/// CONFIGFLAG: every port routed to this controller.
pub const CF_EHCI: u32 = 1;
