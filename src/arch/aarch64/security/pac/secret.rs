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

//! The per-boot secret pointer authentication keys are derived from, taken
//! before the RNG exists: the kernel's key has to be in the register before
//! the first function that signs its return address returns.

use super::rndr::rndr;
use crate::arch::aarch64::cpu::{has_feature, CpuFeature};

const SEED_CONTEXT: &str = "NONOS 2026-10 pointer authentication secret v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Source {
    /// FEAT_RNG on this core.
    Rndr = 1,
    /// The device tree's `/chosen` `rng-seed`, the RNG's own seed, from
    /// which this is derived under its own context.
    DeviceTreeSeed = 2,
}

pub(super) fn boot_secret() -> Option<([u8; 32], Source)> {
    if has_feature(CpuFeature::Rng) {
        if let Some(secret) = from_rndr() {
            return Some((secret, Source::Rndr));
        }
    }
    let secret = crate::arch::aarch64::boot_entropy::derive(SEED_CONTEXT)?;
    Some((secret, Source::DeviceTreeSeed))
}

fn from_rndr() -> Option<[u8; 32]> {
    let mut secret = [0u8; 32];
    for chunk in secret.chunks_exact_mut(8) {
        chunk.copy_from_slice(&rndr()?.to_le_bytes());
    }
    Some(secret)
}

impl Source {
    pub(super) const fn from_status(status: u8) -> Option<Self> {
        match status {
            1 => Some(Source::Rndr),
            2 => Some(Source::DeviceTreeSeed),
            _ => None,
        }
    }
}
