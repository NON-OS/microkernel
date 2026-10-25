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

//! The meter's own secp256k1 key in the keyring. Every request names this
//! process's pid, which the keyring checks against the pid the kernel
//! stamped, so these reach only keys the meter owns.

use nonos_libc::mk_getpid;

use super::call::call;

const OP_WALLET_IMPORT: u16 = 8;
const OP_WALLET_GENERATE: u16 = 9;
const OP_WALLET_ADDRESS: u16 = 10;
const OP_WALLET_EXPORT: u16 = 15;

fn id_of(rx: &[u8]) -> u32 {
    u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]])
}

fn pid_and(tail: &[u8]) -> alloc::vec::Vec<u8> {
    let mut p = alloc::vec::Vec::with_capacity(4 + tail.len());
    p.extend_from_slice(&mk_getpid().to_le_bytes());
    p.extend_from_slice(tail);
    p
}

/// A fresh key, held until this process dies. `now` and an expiry of zero:
/// the meter's key never lapses while the meter runs.
pub(crate) fn generate() -> Result<u32, i32> {
    call(OP_WALLET_GENERATE, &pid_and(&[0u8; 16]), 4).map(|b| id_of(&b))
}

/// The key's secret, to seal it for the next boot. The caller wipes it.
pub(crate) fn export(id: u32) -> Result<[u8; 32], i32> {
    let mut b = call(OP_WALLET_EXPORT, &pid_and(&id.to_le_bytes()), 32)?;
    let mut out = [0u8; 32];
    out.copy_from_slice(&b[8..]);
    crate::wipe::wipe(&mut b);
    Ok(out)
}

/// The sealed secret back into the keyring at boot.
pub(crate) fn import(secret: &[u8; 32]) -> Result<u32, i32> {
    let mut p = pid_and(&[0u8; 16]);
    p.extend_from_slice(secret);
    let r = call(OP_WALLET_IMPORT, &p, 4).map(|b| id_of(&b));
    crate::wipe::wipe(&mut p);
    r
}

pub(crate) fn address(id: u32) -> Result<[u8; 20], i32> {
    let b = call(OP_WALLET_ADDRESS, &pid_and(&id.to_le_bytes()), 20)?;
    let mut out = [0u8; 20];
    out.copy_from_slice(&b[8..]);
    Ok(out)
}
