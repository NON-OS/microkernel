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

//! The three kernel calls behind DeviceSecret, each answer as the pure half
//! takes it. The secret's buffer is the caller's, so the caller wipes it.

use alloc::vec::Vec;

use nonos_libc::boot_slots::{mk_boot_slots, BOOT_SLOTS_LEN};
use nonos_libc::device_secret::mk_device_secret;
use nonos_libc::enroll::{ek_public, EK_ECC_P256, EK_RSA2048, PUBLIC_MAX};

/// The EK answers the TPM gives, P-256 first and RSA 2048 after it, so a
/// device enrolled under either is found; with the last refusal's errno when
/// it gives none.
pub fn ek_answers() -> (Vec<Vec<u8>>, i64) {
    let mut out = Vec::new();
    let mut last = 0;
    for kind in [EK_ECC_P256, EK_RSA2048] {
        let mut buf = [0u8; PUBLIC_MAX];
        let n = ek_public(kind, &mut buf);
        match buf.get(..usize::try_from(n).unwrap_or(0)) {
            Some(a) if n > 0 => out.push(a.to_vec()),
            _ => last = n,
        }
    }
    (out, last)
}

/// The `MkBootSlots` record, or the errno that refused it.
pub fn slots_record() -> Result<[u8; BOOT_SLOTS_LEN], i64> {
    let mut buf = [0u8; BOOT_SLOTS_LEN];
    match mk_boot_slots(&mut buf) {
        n if n == BOOT_SLOTS_LEN as i64 => Ok(buf),
        n if n < 0 => Err(n),
        _ => Err(-5),
    }
}

/// Fill `out` with the secret; 0, or the errno that refused it.
pub fn secret(out: &mut [u8; 32]) -> i64 {
    mk_device_secret(out)
}
