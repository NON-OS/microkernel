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
//! LE Enable Encryption's parameters (Vol 4, Part E, 7.8.24): handle,
//! Random_Number, Encrypted_Diversifier and Long_Term_Key.

use alloc::vec::Vec;

use crate::error::HciError;
use nonos_bt_wire::hci::HANDLE_MAX;
use nonos_bt_wire::WireError;

/// The 28 parameter octets, Rand and EDIV zero as LE Secure Connections
/// has them (Vol 3, Part H, 2.4.4). `ltk` comes most significant octet
/// first, as `nonos_bt_smp` hands it over in `Outcome::Encrypt`; HCI
/// carries it least significant octet first (Vol 4, Part E, 5.2).
pub fn enable_encryption_params(handle: u16, ltk: &[u8; 16]) -> Result<Vec<u8>, HciError> {
    if handle > HANDLE_MAX {
        return Err(HciError::Wire(WireError::Reserved));
    }
    let mut p = Vec::with_capacity(28);
    p.extend_from_slice(&handle.to_le_bytes());
    p.extend_from_slice(&[0; 10]); // Random_Number, then Encrypted_Diversifier
    p.extend(ltk.iter().rev());
    Ok(p)
}
