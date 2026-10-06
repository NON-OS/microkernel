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

//! The verifier's request, read strictly: one length per verifier id, every
//! byte checked, nothing after the id. Integers little-endian.
//!
//! | offset | bytes | field |
//! |---|---|---|
//! | 0 | 8 | `NZKDREQ1` |
//! | 8 | 8 | the window, any u64 |
//! | 16 | 32 | the context nonce: four words, each below p |
//! | 48 | 32 | the device root the verifier accepts: four words, each below p |
//! | 80 | 1 | n, the verifier id's length, 1 to `VERIFIER_MAX` |
//! | 81 | n | the verifier id, bytes 0x21 to 0x7E |
//!
//! The nonce is the statement's context word for word, so no two nonces share
//! one; the id is what the window shows the person before they agree.

use alloc::vec::Vec;

use super::error::Refusal;
use super::words::{le, take};

pub const REQUEST_MAGIC: &[u8; 8] = b"NZKDREQ1";
pub const VERIFIER_MAX: usize = 128;
const HEAD: usize = 81;
pub const REQUEST_MAX: usize = HEAD + VERIFIER_MAX;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub window: u64,
    pub nonce: [u8; 32],
    pub device_root: [u8; 32],
    pub verifier: Vec<u8>,
}

pub fn parse_request(r: &[u8]) -> Result<Request, Refusal> {
    if r.len() < HEAD || r.len() > REQUEST_MAX {
        return Err(Refusal::RequestSize);
    }
    if r.get(..8) != Some(REQUEST_MAGIC.as_slice()) {
        return Err(Refusal::RequestMagic);
    }
    let n = usize::from(*r.get(80).ok_or(Refusal::RequestSize)?);
    if n == 0 || n > VERIFIER_MAX {
        return Err(Refusal::RequestVerifierLength);
    }
    if r.len() != HEAD + n {
        return Err(Refusal::RequestSize);
    }
    let window = u64::from_le_bytes(le(r, 8).ok_or(Refusal::RequestSize)?);
    let nonce = take(r, 16).ok_or(Refusal::RequestNonceWord)?;
    let device_root = take(r, 48).ok_or(Refusal::RequestRootWord)?;
    let verifier = r.get(HEAD..).ok_or(Refusal::RequestSize)?;
    if !verifier.iter().all(|b| (0x21..=0x7E).contains(b)) {
        return Err(Refusal::RequestVerifierByte);
    }
    Ok(Request { window, nonce, device_root, verifier: verifier.to_vec() })
}
