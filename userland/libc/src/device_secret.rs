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

//! This machine's device secret, the witness of the anonymous device proof.

use crate::syscall::{call_raw, N_MK_DEVICE_SECRET};

/// Fill `out` with this machine's TPM-derived device secret, four field words
/// little-endian. Needs DeviceSecret, which nonos.prove alone holds. 0; ENOENT
/// while the secret stays sealed, EACCES on a chain the TPM's policy refuses,
/// ENODEV when the TPM cannot derive it.
pub fn mk_device_secret(out: &mut [u8; 32]) -> i64 {
    call_raw(N_MK_DEVICE_SECRET, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}
