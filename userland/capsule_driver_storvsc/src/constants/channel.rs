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

//! The channel's record, as the VMBus core has the kernel make it: vendor
//! 0x1414 (Microsoft) and the SCSI interface, its relid in `irq_source`.

pub const VMBUS_VENDOR: u16 = 0x1414;
/// The SCSI controller interface (ba6163d9-04a1-4d29-b605-72e2ffb1dc7f).
pub const SCSI_DEVICE: u16 = 0x5343;
/// Targets 0 and 1, LUNs 0 to 254, as Linux's STORVSC_MAX_TARGETS and
/// STORVSC_MAX_LUNS_PER_TARGET bound them (both one past the last).
pub const MAX_TARGET: u16 = 1;
pub const MAX_LUN: u16 = 254;
/// The open's user data: Linux hands a zeroed vmstorage_channel_properties.
pub const OPEN_USER: [u8; 24] = [0; 24];
/// The GPADL's id: "SC" and the relid, so no two channels' GPADLs share one.
pub const fn gpadl_id(relid: u32) -> u32 {
    0x5343_0000 | (relid & 0xFFFF)
}
