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

//! The stand-in attestation document: `"TDOC"
//!  challenge
//!  AK
//!  PCRs`.

use super::{AK, PCRS};

pub const DOC_LEN: usize = 4 + 32 + 64 + 4 * 32;

pub fn doc_with(challenge: &[u8; 32], ak: &[u8; 64], pcrs: &[[u8; 32]; 4]) -> Vec<u8> {
    let mut d = b"TDOC".to_vec();
    d.extend_from_slice(challenge);
    d.extend_from_slice(ak);
    pcrs.iter().for_each(|p| d.extend_from_slice(p));
    d
}

pub fn doc(challenge: &[u8; 32]) -> Vec<u8> {
    doc_with(challenge, &AK, &PCRS)
}
