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

//! What the tests ask of the kernel's readers, from inside the module the
//! kernel keeps their items private to.

use super::key_header::{parse_key_header, Keyed, AAD_END, KEY_LBA};
use super::plan::parse_plan;
pub use super::plan_types::PlanError;

pub const KERNEL_KEY_LBA: u64 = KEY_LBA;
/// Where the bytes the kernel's seal authenticates end.
pub const KERNEL_AAD_END: usize = AAD_END;

/// The volume a plan sector names on a disk of `capacity` sectors, and how
/// many files it asks the kernel to import.
pub fn plan_of(sector: &[u8; 512], capacity: u64) -> Result<(u64, u64, usize), PlanError> {
    parse_plan(sector, capacity).map(|p| (p.volume_base, p.volume_sectors, p.imports().len()))
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyedBy {
    NoHeader,
    Tpm,
    Passphrase { cost: [u32; 3], salt: [u8; 32], nonce: [u8; 12], sealed: [u8; 48] },
    Unknown(u8),
}

/// How the kernel takes a key header sector.
pub fn keyed_by(s: &[u8; 512]) -> KeyedBy {
    match parse_key_header(s) {
        Ok(None) => KeyedBy::NoHeader,
        Ok(Some(Keyed::Tpm)) => KeyedBy::Tpm,
        Ok(Some(Keyed::Passphrase(k))) => KeyedBy::Passphrase {
            cost: [k.params.m_kib, k.params.t, k.params.p],
            salt: k.salt,
            nonce: k.nonce,
            sealed: k.key_and_tag,
        },
        Err(way) => KeyedBy::Unknown(way),
    }
}
