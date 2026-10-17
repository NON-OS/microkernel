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

//! The register bits a reset reads and writes (L78), as pure values. Every
//! one is recalled from the PCIe base specification and unverified: a wrong
//! one ends a wipe Unreset or Lost, never in a false Clean.

/// Device Capabilities: Function Level Reset offered.
pub(crate) const FLR_CAP: u32 = 1 << 28;
/// Device Control: Initiate FLR; Enable No Snoop, put back cleared.
pub(crate) const INITIATE_FLR: u16 = 1 << 15;
pub(crate) const NO_SNOOP_EN: u16 = 1 << 11;
/// Device Status: Transactions Pending.
pub(crate) const TRANSACTIONS_PENDING: u16 = 1 << 5;
/// Advanced Features: its capabilities (+3), control (+4) and status (+5).
pub(crate) const AF_TP_CAP: u8 = 1;
pub(crate) const AF_FLR_CAP: u8 = 2;
pub(crate) const AF_INITIATE_FLR: u8 = 1;
pub(crate) const AF_TP: u8 = 1;
/// Power Management Control/Status.
pub(crate) const PM_STATE: u16 = 0x3;
pub(crate) const D3HOT: u16 = 3;
pub(crate) const NO_SOFT_RESET: u16 = 1 << 3;
pub(crate) const PME_STATUS: u16 = 1 << 15;
/// Command: the two decoders, Bus Master and INTx Disable.
pub(crate) const IO: u16 = 1;
pub(crate) const MEM: u16 = 1 << 1;
pub(crate) const BME: u16 = 1 << 2;
pub(crate) const INTX_DISABLE: u16 = 1 << 10;

/// The Vendor ID a function not yet back gives: all ones, or the retry
/// status value 0x0001.
pub(crate) const fn not_ready(id: u32) -> bool {
    matches!(id & 0xFFFF, 0xFFFF | 0x0001)
}

/// Device Control as put back: never a reset asked again, never no-snoop.
pub(crate) const fn devctl_after(before: u16) -> u16 {
    before & !(INITIATE_FLR | NO_SNOOP_EN)
}

/// Device Control 2's completion timeout value and disable, which a network
/// holder may write (L80).
pub(crate) const CT_BITS: u16 = 0x001F;

/// Device Control 2 as put back: its completion timeout as firmware left it
/// at enumeration, `boot`, never as the last holder set it (L80).
pub(crate) const fn devctl2_after(before: u16, boot: u16) -> u16 {
    (before & !CT_BITS) | (boot & CT_BITS)
}

/// PMCSR moved to `state`, PME_Status (write one to clear) written 0.
pub(crate) const fn pmcsr_to(read: u16, state: u16) -> u16 {
    (read & !(PM_STATE | PME_STATUS)) | (state & PM_STATE)
}
