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

//! The ACPI TPM2 table from its bytes, with no pointer read: `acpi.rs`
//! hands it the table and keeps the answer for whoever drives the TPM;
//! `kernel_proofs` mounts it by path. After the 36-byte header: the
//! platform class u16, two reserved bytes, the CRB control area's address
//! u64, the start method u32, then the start method's parameters (12
//! bytes in the tables that go on) and, when the table is long enough
//! for it, the event log area's minimum length u32 and start address u64.
//! Recalled from the TCG ACPI specification, not read: unverified.

use alloc::vec::Vec;

pub const MIN_LEN: usize = 52;
/// No revision of the table comes near this; a larger length is corrupt.
pub const MAX_LEN: usize = 4096;
const PARAMETERS_AT: usize = 52;
const LOG_AT: usize = 64;
const WITH_LOG: usize = 76;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tpm2Table {
    /// 0 client, 1 server.
    pub platform_class: u16,
    pub control_area: u64,
    pub start_method: u32,
    pub parameters: Vec<u8>,
    /// The event log area's minimum length and address.
    pub log_area: Option<(u32, u64)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm2Fault {
    Length(usize),
    /// What the bytes sum to; a sound table sums to 0.
    Checksum(u8),
}

pub fn decode(t: &[u8]) -> Result<Tpm2Table, Tpm2Fault> {
    let len = t.get(4..8).map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let t = t.get(..len).filter(|_| (MIN_LEN..=MAX_LEN).contains(&len));
    let t = t.ok_or(Tpm2Fault::Length(len))?;
    let sum = t.iter().fold(0u8, |s, b| s.wrapping_add(*b));
    if sum != 0 {
        return Err(Tpm2Fault::Checksum(sum));
    }
    let u32_at = |at: usize| u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]]);
    let u64_at = |at: usize| u64::from(u32_at(at)) | u64::from(u32_at(at + 4)) << 32;
    let log_area = (len >= WITH_LOG).then(|| (u32_at(LOG_AT), u64_at(LOG_AT + 4)));
    let parameters = t[PARAMETERS_AT..if log_area.is_some() { LOG_AT } else { len }].to_vec();
    Ok(Tpm2Table {
        platform_class: u16::from_le_bytes([t[36], t[37]]),
        control_area: u64_at(40),
        start_method: u32_at(48),
        parameters,
        log_area,
    })
}
