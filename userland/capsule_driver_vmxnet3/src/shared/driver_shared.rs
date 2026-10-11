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

//! Vmxnet3_DriverShared, revision 1: what the device reads on ACTIVATE_DEV.
//! Offsets from vmxnet3_defs.h; devRead starts at 8, misc is 72 bytes,
//! intrConf 40, rxFilterConf 528.

use super::area::Area;
use crate::constants::cmd::{RXM_BCAST, RXM_UCAST};
use crate::constants::layout::QUEUE_DESC_LEN;

/// The whole structure, devReadExt included.
pub const SHARED_LEN: usize = 800;
const REV1_MAGIC: u32 = 0xBABE_FEE1;
const _: () = assert!(SHARED_LEN <= crate::constants::layout::TXQ_DESC_OFF);
const MISC: usize = 8;
const INTR: usize = MISC + 72;
const RX_FILTER: usize = INTR + 40;
/// The device only logs it.
const DRIVER_VERSION: u32 = 0x0001_0000;
/// gosBits 2 (64-bit), gosType 0: NONOS is none of the systems it names.
const GOS_64BIT: u32 = 2;
const IC_DISABLE_ALL: u32 = 1;
const MTU: u32 = 1500;

pub struct SharedConfig {
    pub queue_desc_pa: u64,
    /// Driver data, which the device only keeps for its own diagnostics.
    pub dd_pa: u64,
    pub dd_len: u32,
}

pub fn write_shared(a: &mut Area, cfg: &SharedConfig) {
    a.u32(0, REV1_MAGIC);
    a.u32(MISC, DRIVER_VERSION);
    a.u32(MISC + 4, GOS_64BIT);
    a.u32(MISC + 8, 1); // vmxnet3RevSpt
    a.u32(MISC + 12, 1); // uptVerSpt
                         // uptFeatures at MISC + 16 stays zero: no checksum offload, LRO, VLAN
                         // stripping or RSS, so every frame arrives as it was on the wire.
    a.u64(MISC + 24, cfg.dd_pa);
    a.u64(MISC + 32, cfg.queue_desc_pa);
    a.u32(MISC + 40, cfg.dd_len);
    a.u32(MISC + 44, QUEUE_DESC_LEN as u32);
    a.u32(MISC + 48, MTU);
    a.u8(MISC + 54, 1); // numTxQueues
    a.u8(MISC + 55, 1); // numRxQueues
    a.u8(INTR, 1); // autoMask
    a.u8(INTR + 1, 1); // numIntrs
    a.u32(INTR + 28, IC_DISABLE_ALL);
    a.u32(RX_FILTER, RXM_UCAST | RXM_BCAST);
    // vfTable bit 0 is VLAN 0, untagged frames; Linux always sets it.
    a.u32(RX_FILTER + 16, 1);
}
