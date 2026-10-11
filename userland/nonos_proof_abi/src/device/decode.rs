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

//! The device proof section: the prover's NZKDPRF1 file as it left
//! nonos.prove. The magic, the boot, kernel and device roots at 8, 40 and 72,
//! depth u32 at 104, scope at 108, context at 124, tag at 156, the proof's
//! length L at 188, and from 192 the proof's own wire bytes.

use alloc::vec::Vec;

use nonos_device_attest::{Proof, Statement, MAX_DEVICE_DEPTH};
use nonos_stark::field::{Fp, P};

use crate::words::{b32_at, u32_at, u64_at, words_of_le};

use super::layout::{DEVICE_HEAD, DEVICE_MAGIC, DEVICE_MAX};

/// Four canonical words at `at`; a word at or above p is refused rather than
/// reduced, so one statement has one encoding.
fn words(b: &[u8], at: usize) -> Option<[Fp; 4]> {
    let w = words_of_le(&b32_at(b, at)?);
    if w.iter().any(|&v| v >= P) {
        return None;
    }
    Some(w.map(Fp::from_u64))
}

pub fn decode_device(b: &[u8]) -> Option<(Statement, Proof)> {
    if b.len() <= DEVICE_HEAD || b.len() > DEVICE_MAX || b.get(..8)? != DEVICE_MAGIC {
        return None;
    }
    let depth = u32_at(b, 104)? as usize;
    let len = u32_at(b, 188)? as usize;
    if depth == 0 || depth > MAX_DEVICE_DEPTH || b.len() != DEVICE_HEAD.checked_add(len)? {
        return None;
    }
    let scope = [u64_at(b, 108)?, u64_at(b, 116)?];
    if scope.iter().any(|&v| v >= P) {
        return None;
    }
    let statement = Statement {
        boot_root: words(b, 8)?,
        kernel_root: words(b, 40)?,
        device_root: words(b, 72)?,
        device_depth: depth,
        scope: scope.map(Fp::from_u64),
        context: words(b, 124)?,
        tag: words(b, 156)?,
    };
    let proof: Vec<u8> = b[DEVICE_HEAD..].to_vec();
    Some((statement, Proof { bytes: proof }))
}
