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

//! The output file read back, as an export or a verifier reads it: every
//! word canonical, the depth one the circuit takes, the length exact. What it
//! gives is handed to `verify` as is; nothing in it is trusted before that.

use nonos_device_attest::{words_of, Proof, Statement, MAX_DEVICE_DEPTH};

use super::output::{OUTPUT_HEAD, OUTPUT_MAGIC, OUTPUT_MAX};
use super::words::{le, take};

pub struct Decoded {
    pub statement: Statement,
    pub proof: Proof,
}

pub fn decode(b: &[u8]) -> Option<Decoded> {
    if b.len() <= OUTPUT_HEAD || b.len() > OUTPUT_MAX || b.get(..8)? != OUTPUT_MAGIC {
        return None;
    }
    let depth = u32::from_le_bytes(le(b, 104)?) as usize;
    if depth == 0 || depth > MAX_DEVICE_DEPTH {
        return None;
    }
    let mut scope = [0u8; 32];
    scope[..16].copy_from_slice(b.get(108..124)?);
    let scope = words_of(&take(&scope, 0)?);
    let len = u32::from_le_bytes(le(b, 188)?) as usize;
    if b.len() != OUTPUT_HEAD.checked_add(len)? {
        return None;
    }
    let statement = Statement {
        boot_root: words_of(&take(b, 8)?),
        kernel_root: words_of(&take(b, 40)?),
        device_root: words_of(&take(b, 72)?),
        device_depth: depth,
        scope: [scope[0], scope[1]],
        context: words_of(&take(b, 124)?),
        tag: words_of(&take(b, 156)?),
    };
    Some(Decoded { statement, proof: Proof { bytes: b.get(OUTPUT_HEAD..)?.to_vec() } })
}
