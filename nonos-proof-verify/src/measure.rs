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

//! Measurements and the contexts the gates build from them, byte for byte.

use nonos_attest_path::{boot_context, capsule_context, Kind};

/// The capsule spawn gate's policy epoch (src/security/capsule_attest/layout.rs).
pub const POLICY_EPOCH: u64 = 1;
/// The kernel's and the loader's boot epoch.
pub const BOOT_EPOCH: u64 = 1;

/// What a slot enrolls: a measurement and, for a capsule, the capability word
/// the spawn gate grants it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Kernel([u8; 32]),
    Capsule([u8; 32], u64),
    Bootloader([u8; 32]),
}

impl Slot {
    pub fn kind(&self) -> Kind {
        match self {
            Slot::Kernel(_) => Kind::Kernel,
            Slot::Capsule(..) => Kind::Capsule,
            Slot::Bootloader(_) => Kind::Bootloader,
        }
    }

    pub fn measurement(&self) -> &[u8; 32] {
        match self {
            Slot::Kernel(m) | Slot::Capsule(m, _) | Slot::Bootloader(m) => m,
        }
    }

    pub fn caps(&self) -> Option<u64> {
        match self {
            Slot::Capsule(_, c) => Some(*c),
            _ => None,
        }
    }

    /// The context the slot's gate builds.
    pub fn context(&self) -> Vec<u8> {
        match self {
            Slot::Kernel(m) | Slot::Bootloader(m) => boot_context(m, BOOT_EPOCH).to_vec(),
            Slot::Capsule(m, c) => capsule_context(m, *c, POLICY_EPOCH).to_vec(),
        }
    }
}

/// A capsule's or a kernel's measurement: BLAKE3 over the image's bytes.
pub fn blake3_of(image: &[u8]) -> [u8; 32] {
    *blake3::hash(image).as_bytes()
}

/// A loader's measurement: the PE Authenticode SHA-256 the firmware extends
/// PCR 4 with, by the boot gate's own reader.
pub fn authenticode_of(image: &[u8]) -> Result<[u8; 32], String> {
    nonos_boot_measure::authenticode::digest(image)
        .map_err(|e| format!("no Authenticode digest: {e:?} (code {})", e.code()))
}
