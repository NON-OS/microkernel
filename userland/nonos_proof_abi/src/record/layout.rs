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

//! NFSR v1: the kernel's sealed record of one supervisor family, 240 bytes,
//! little-endian. Everything in it is public; the histogram, its salt and the
//! supervisor's diagnostic are not in it, only the commitment Hc.

pub const RECORD_BYTES: usize = 240;
pub(crate) const MAGIC: &[u8; 4] = b"NFSR";
pub(crate) const VERSION: u16 = 1;

/// Sealed after the last guest, covering every call since the first.
pub const FLAG_FINAL: u16 = 1;
/// Some count saturated: the counts are lower bounds only.
pub const FLAG_SATURATED: u16 = 2;
/// More than 4096 distinct measurements: M stopped absorbing.
pub const FLAG_OVERFLOW: u16 = 4;

pub(super) const FLAGS: usize = 6;
pub(super) const SUP_MEAS: usize = 8;
pub(super) const SUP_CAPS: usize = 40;
pub(super) const AUTHORITY: usize = 48;
pub(super) const SEAL_NO: usize = 56;
pub(super) const TRACE: usize = 64;
pub(super) const ROWS: usize = 96;
pub(super) const HC: usize = 104;
pub(super) const MEAS: usize = 136;
pub(super) const N_MEAS: usize = 168;
pub(super) const APP_ID: usize = 176;

/// The reserved spans, each zero in a well-formed record. The last is where
/// a diagnostic would be, and it never is.
pub(super) const RESERVED: [(usize, usize); 3] = [(49, 56), (172, 176), (208, 240)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Record {
    pub flags: u16,
    /// The supervisor's measurement, as the attest registry holds it.
    pub sup_meas: [u8; 32],
    pub sup_caps: u64,
    /// The registry's authority byte: 0 vendor, 1 + slot developer, 255
    /// publisher.
    pub authority: u8,
    /// A random handle, stable for the family.
    pub seal_no: u64,
    /// K, the head of the family's trace chain.
    pub trace: [u8; 32],
    /// How many rows K absorbed.
    pub rows: u64,
    /// The histogram commitment, four field words.
    pub hc: [u64; 4],
    /// M, the head of the measurement chain, and how many links it has.
    pub meas: [u8; 32],
    pub n_meas: u32,
    /// Slot 0, written once by the supervisor; zero when unset.
    pub app_id: [u8; 32],
}
