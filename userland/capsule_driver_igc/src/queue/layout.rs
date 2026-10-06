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

//! Advanced descriptor layouts from igc_base.h. Both are 16 bytes; the fields
//! are named for the quadwords as both the driver and the part use them.

/// union igc_adv_rx_desc. The driver writes the read format: packet buffer
/// address, then a header buffer address that is zero with one buffer per
/// descriptor. The part writes back over all sixteen bytes, and the second
/// quadword becomes wb.upper: status_error, length, vlan. Those are the
/// fields here, so zeroing them is writing hdr_addr zero for the next pass.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RxDesc {
    pub buffer_addr: u64,
    pub staterr: u32,
    pub length: u16,
    pub vlan: u16,
}

/// union igc_adv_tx_desc. The part writes its completion status (wb.status,
/// DD in bit 0) over olinfo_status.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TxDesc {
    pub buffer_addr: u64,
    pub cmd_type_len: u32,
    pub olinfo_status: u32,
}

const _: () = assert!(core::mem::size_of::<RxDesc>() == 16);
const _: () = assert!(core::mem::size_of::<TxDesc>() == 16);
