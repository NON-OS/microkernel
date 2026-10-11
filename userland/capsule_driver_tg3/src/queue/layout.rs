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

//! struct tg3_rx_buffer_desc and struct tg3_tx_buffer_desc (tg3.h), field
//! for field. With the non-frame word swap on, each field is a native u32.

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RxBd {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub idx_len: u32,
    pub type_flags: u32,
    pub ip_tcp_csum: u32,
    pub err_vlan: u32,
    pub reserved: u32,
    pub opaque: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TxBd {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub len_flags: u32,
    pub vlan_tag: u32,
}

const _: () = assert!(core::mem::size_of::<RxBd>() == crate::constants::desc::RX_BD_BYTES);
const _: () = assert!(core::mem::size_of::<TxBd>() == crate::constants::desc::TX_BD_BYTES);
