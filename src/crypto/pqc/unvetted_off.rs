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

//! SPHINCS+, NTRU and Classic McEliece are not built (see `unvetted`, under
//! the `pqc-unvetted` feature). Their wrappers refuse here, by name, so a
//! caller learns the algorithm is not shipped rather than meeting a missing
//! symbol.

extern crate alloc;
use alloc::vec::Vec;

const NOT_SHIPPED: &str = "algorithm not shipped: it has not passed its published vectors";

type Pair = Result<(Vec<u8>, Vec<u8>), &'static str>;

pub fn sphincs128s_keypair() -> Pair {
    Err(NOT_SHIPPED)
}
pub fn sphincs128s_sign(_message: &[u8], _sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    Err(NOT_SHIPPED)
}
pub fn sphincs128s_verify(_message: &[u8], _sig: &[u8], _pk: &[u8]) -> bool {
    false
}
pub fn ntruhps4096821_keypair() -> Pair {
    Err(NOT_SHIPPED)
}
pub fn ntruhps4096821_encapsulate(_pk: &[u8]) -> Pair {
    Err(NOT_SHIPPED)
}
pub fn ntruhps4096821_decapsulate(_ct: &[u8], _sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    Err(NOT_SHIPPED)
}
pub fn mceliece348864_keypair() -> Pair {
    Err(NOT_SHIPPED)
}
pub fn mceliece348864_encapsulate(_pk: &[u8]) -> Pair {
    Err(NOT_SHIPPED)
}
pub fn mceliece348864_decapsulate(_ct: &[u8], _sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    Err(NOT_SHIPPED)
}
