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

//! What a proof leaves on the data volume for the fetcher or an export to
//! carry: the statement's public values, then the proof, and nothing of the
//! witness. Integers little-endian; every word a field word, below p.
//!
//! | offset | bytes | field |
//! |---|---|---|
//! | 0 | 8 | `NZKDPRF1` |
//! | 8 | 32 | boot_root |
//! | 40 | 32 | kernel_root |
//! | 72 | 32 | device_root |
//! | 104 | 4 | device_depth, 1 to `MAX_DEVICE_DEPTH` |
//! | 108 | 16 | scope, two words |
//! | 124 | 32 | context |
//! | 156 | 32 | tag |
//! | 188 | 4 | L, the proof's length |
//! | 192 | L | the proof: `nonos-device-attest`'s wire bytes, header included |
//!
//! The whole file is at most `OUTPUT_MAX`, the most `CryptoHash` digests in
//! one call, since the volume keeps a file only under its SHA-256.

use alloc::vec::Vec;

use nonos_device_attest::{Proof, Statement};

use super::error::Refusal;
use super::words::put;

pub const OUTPUT_MAGIC: &[u8; 8] = b"NZKDPRF1";
pub const OUTPUT_HEAD: usize = 192;
pub const OUTPUT_MAX: usize = 1 << 20;

pub fn encode(st: &Statement, proof: &Proof) -> Result<Vec<u8>, Refusal> {
    let n = proof.bytes.len();
    if n > OUTPUT_MAX - OUTPUT_HEAD {
        return Err(Refusal::OutputSize);
    }
    let len = u32::try_from(n).map_err(|_| Refusal::OutputSize)?;
    let depth = u32::try_from(st.device_depth).map_err(|_| Refusal::OutputSize)?;
    let mut out = Vec::with_capacity(OUTPUT_HEAD + n);
    out.extend_from_slice(OUTPUT_MAGIC);
    let roots = st.boot_root.iter().chain(&st.kernel_root).chain(&st.device_root);
    put(&mut out, roots.map(|w| w.to_u64()));
    out.extend_from_slice(&depth.to_le_bytes());
    let rest = st.scope.iter().chain(&st.context).chain(&st.tag);
    put(&mut out, rest.map(|w| w.to_u64()));
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&proof.bytes);
    Ok(out)
}
