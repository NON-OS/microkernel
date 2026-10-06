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

//! The two words of an advanced transmit data descriptor for one whole frame
//! in one buffer. igc_tx_cmd_type sets DTYP_DATA, DEXT and IFCS; IGC_TXD_DCMD
//! adds EOP and RS on the last descriptor, and the buffer length sits in the
//! low 16 bits (igc_tx_map). igc_tx_olinfo_status puts the payload length,
//! the whole frame without TSO, at PAYLEN_SHIFT in olinfo_status.

use crate::constants::tx_bits::{
    ADVTXD_DCMD_DEXT, ADVTXD_DCMD_EOP, ADVTXD_DCMD_IFCS, ADVTXD_DCMD_RS, ADVTXD_DTYP_DATA,
    ADVTXD_LEN_MASK, ADVTXD_PAYLEN_SHIFT,
};

pub fn cmd_type_len(len: u16) -> u32 {
    ADVTXD_DTYP_DATA
        | ADVTXD_DCMD_DEXT
        | ADVTXD_DCMD_IFCS
        | ADVTXD_DCMD_EOP
        | ADVTXD_DCMD_RS
        | (len as u32 & ADVTXD_LEN_MASK)
}

pub fn olinfo_status(len: u16) -> u32 {
    (len as u32) << ADVTXD_PAYLEN_SHIFT
}
