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

//! The registry a transcript rebuilds, and this device's place in it. The
//! root is held to its entries by `Registry::recompute`, then to what the
//! caller accepts. The device is found by `ek_id` over its EK's TPM2B_PUBLIC,
//! the bytes the registrar keys on; only the path and commitment are kept.

use nonos_device_attest::{ek_id, words_of, Path, Registry};
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::error::Refusal;
use super::request::Request;
use super::transcript_line::commitment_of;
use crate::abi::split_public;

/// A full registry at the deepest the circuit takes, a million devices of
/// about 160 bytes each, with room for its revocations.
pub const TRANSCRIPT_MAX: usize = 192 << 20;

pub struct Enrolled {
    /// The registry's depth, the statement's `device_depth`.
    pub depth: usize,
    /// The path of this device's commitment to the root.
    pub path: Path,
    /// The commitment the registry holds for this device, four words.
    pub commitment: [u8; 32],
}

/// `eks` are the TPM's EK answers in the order to try them.
pub fn enrolled(transcript: &[u8], req: &Request, eks: &[&[u8]]) -> Result<Enrolled, Refusal> {
    let want = words_of(&req.device_root);
    enrolled_where(transcript, |root, _| root == want, eks)
}

/// Under any test of the rebuilt root and depth: the market's (plan/14).
pub fn enrolled_where<F>(transcript: &[u8], accept: F, eks: &[&[u8]]) -> Result<Enrolled, Refusal>
where
    F: Fn([Fp; RATE], usize) -> bool,
{
    if transcript.len() > TRANSCRIPT_MAX {
        return Err(Refusal::TranscriptSize);
    }
    let text = core::str::from_utf8(transcript).map_err(|_| Refusal::TranscriptText)?;
    let reg = Registry::recompute(text).map_err(Refusal::Transcript)?;
    if !accept(reg.root(), reg.depth()) {
        return Err(Refusal::RootNotRequested);
    }
    if eks.is_empty() {
        return Err(Refusal::NoEk);
    }
    for answer in eks {
        let (_, area) = split_public(answer).ok_or(Refusal::EkAnswer)?;
        let id = ek_id(area);
        if let Some(path) = reg.path(&id) {
            let commitment = commitment_of(text, &id).ok_or(Refusal::NotEnrolled)?;
            return Ok(Enrolled { depth: reg.depth(), path, commitment });
        }
    }
    Err(Refusal::NotEnrolled)
}
