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

use alloc::vec::Vec;

use nonos_device_attest::{Proof, Statement};
use nonos_stark::field::Fp;

use super::layout::{DEVICE_HEAD, DEVICE_MAGIC, DEVICE_MAX};

fn put(out: &mut Vec<u8>, words: &[Fp]) {
    words.iter().for_each(|w| out.extend_from_slice(&w.to_u64().to_le_bytes()));
}

/// The section `decode_device` reads. `None` for an empty proof or one that
/// would take the section past its bound.
pub fn encode_device(st: &Statement, proof: &Proof) -> Option<Vec<u8>> {
    let n = proof.bytes.len();
    if n == 0 || n > DEVICE_MAX - DEVICE_HEAD {
        return None;
    }
    let depth = u32::try_from(st.device_depth).ok()?;
    let mut out = Vec::with_capacity(DEVICE_HEAD + n);
    out.extend_from_slice(DEVICE_MAGIC);
    put(&mut out, &st.boot_root);
    put(&mut out, &st.kernel_root);
    put(&mut out, &st.device_root);
    out.extend_from_slice(&depth.to_le_bytes());
    put(&mut out, &st.scope);
    put(&mut out, &st.context);
    put(&mut out, &st.tag);
    out.extend_from_slice(&(n as u32).to_le_bytes());
    out.extend_from_slice(&proof.bytes);
    Some(out)
}
