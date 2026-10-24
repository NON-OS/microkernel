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

//! `TPMT_SIGNATURE` and the `TPM2B` sized field.

pub fn tpm2b(out: &mut Vec<u8>, v: &[u8]) {
    out.extend((v.len() as u16).to_be_bytes());
    out.extend(v);
}

/// `TPMT_SIGNATURE`: scheme, hash, then r and s each after its size.
pub fn tpmt(alg: u16, hash: u16, r: &[u8], s: &[u8]) -> Vec<u8> {
    let mut out = alg.to_be_bytes().to_vec();
    out.extend(hash.to_be_bytes());
    tpm2b(&mut out, r);
    tpm2b(&mut out, s);
    out
}
