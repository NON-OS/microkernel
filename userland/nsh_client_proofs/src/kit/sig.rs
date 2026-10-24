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

//! Hybrid signatures for the proofs, with either half spoilt on demand.

use super::keys::Hybrid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigMode {
    Good,
    /// One byte of the Ed25519 half changed; the ML-DSA-65 half is good.
    BadEd,
    /// One byte of the ML-DSA-65 half changed; the Ed25519 half is good.
    BadMl,
}

/// `msg` signed by `key`, spoilt as `mode` says.
pub fn signed(key: &Hybrid, msg: &[u8], mode: SigMode) -> Vec<u8> {
    let mut sig = key.sign(msg);
    match mode {
        SigMode::Good => {}
        SigMode::BadEd => sig[10] ^= 1,
        SigMode::BadMl => sig[64 + 100] ^= 1,
    }
    sig
}
