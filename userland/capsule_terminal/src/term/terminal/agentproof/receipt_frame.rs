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

//! The frames a headless run asks nonos.prove for its receipt in (letter
//! agent-02): nonos_proof_abi::wire's 20-byte header under "NPRV", written out
//! here so the Terminal does not carry the prover's crate. prove_proofs holds
//! every byte of it equal to that crate's.
//!
//!   6 RECEIPT      box_len u8 | box | policy[256]  -> status | NAPR v0
//!   7 RECEIPT_ACK  seal_no u64 LE                  -> status

use alloc::vec::Vec;

pub const PROVE_MAGIC: u32 = 0x5652_504E;
pub const RECEIPT: u16 = 6;
pub const RECEIPT_ACK: u16 = 7;
pub const HDR_LEN: usize = 20;
/// The largest reply: a header, a status and the 64 KiB a frame carries.
pub const FRAME_MAX: usize = HDR_LEN + 4 + 64 * 1024;
const VERSION: u16 = 1;
/// "NAPR", version 0 and the record's length word; then, in the NFSR v1
/// record, its seal number, little-endian.
const HEAD: &[u8; 10] = b"NAPR\0\0\0\0\0\xf0";
const SEAL_NO_AT: usize = HEAD.len() + 56;

pub fn request(op: u16, id: u32, body: &[u8]) -> Vec<u8> {
    let mut b = Vec::with_capacity(HDR_LEN + body.len());
    b.extend_from_slice(&PROVE_MAGIC.to_le_bytes());
    b.extend_from_slice(&VERSION.to_le_bytes());
    b.extend_from_slice(&op.to_le_bytes());
    b.extend_from_slice(&[0; 4]);
    b.extend_from_slice(&id.to_le_bytes());
    b.extend_from_slice(&(body.len() as u32).to_le_bytes());
    b.extend_from_slice(body);
    b
}

/// A RECEIPT frame's body; None for a box word the frame cannot carry.
pub fn receipt_body(box_word: &[u8], policy: &[u8; 256]) -> Option<Vec<u8>> {
    let len = u8::try_from(box_word.len()).ok().filter(|&n| n > 0)?;
    Some([&[len][..], box_word, &policy[..]].concat())
}

/// The status and body of the reply to `op` and `id`; None for any other
/// frame, or one whose length is not exactly what it holds.
pub fn reply(rx: &[u8], op: u16, id: u32) -> Option<(i32, &[u8])> {
    let at16 = |i: usize| Some(u16::from_le_bytes(rx.get(i..i + 2)?.try_into().ok()?));
    let at32 = |i: usize| Some(u32::from_le_bytes(rx.get(i..i + 4)?.try_into().ok()?));
    let fixed = at32(0)? == PROVE_MAGIC && at16(4)? == VERSION && at16(10)? == 0;
    if !fixed || at16(6)? != op || at32(12)? != id || at32(16)? as usize != rx.len() - HDR_LEN {
        return None;
    }
    Some((at32(HDR_LEN)? as i32, rx.get(HDR_LEN + 4..)?))
}

/// The seal number of the record a receipt carries, which its ack names.
pub fn seal_no(napr: &[u8]) -> Option<u64> {
    let at = SEAL_NO_AT;
    (napr.get(..HEAD.len())? == HEAD).then_some(())?;
    Some(u64::from_le_bytes(napr.get(at..at + 8)?.try_into().ok()?))
}
