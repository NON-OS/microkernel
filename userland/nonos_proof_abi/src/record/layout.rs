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
//! little-endian. Everything in it is public; the histogram and its salt are
//! not in it, only their commitment Hc.

pub const RECORD_BYTES: usize = 240;
pub(crate) const MAGIC: &[u8; 4] = b"NFSR";
pub(crate) const VERSION: u16 = 1;

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
pub(super) const DIAG: usize = 208;

/// The reserved spans, each zero in a well-formed record.
pub(super) const RESERVED: [(usize, usize); 3] = [(6, 8), (49, 56), (172, 176)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Record {
    /// The supervisor's measurement, as the attest registry holds it.
    pub sup_meas: [u8; 32],
    pub sup_caps: u64,
    /// The attest registry's authority byte.
    pub authority: u8,
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
    /// Slot 1, the supervisor's own diagnostic head; zero when unset.
    pub diag: [u8; 32],
}
