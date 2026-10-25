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

//! The pin init writes for an escrow install, in place of the package's
//! BLAKE3: `e`, then 64 hex of the release's chunk_root, 16 hex of
//! its content_len as a big-endian u64, and 64 hex of the order id, 145
//! lowercase characters in all. Anything else is refused. Pure, so the host
//! proofs hold it; the install role reads it at integration (I-C5).

/// The pin's length in characters.
pub const ESCROW_PIN_LEN: usize = 1 + 64 + 16 + 64;

/// What an escrow pin names: the bytes the install must find, and the order
/// whose delivery the meter hands over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EscrowPin {
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub order_id: [u8; 32],
}

/// A word that is not an escrow pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotEscrowPin;

pub fn escrow_pin(word: &[u8]) -> Result<EscrowPin, NotEscrowPin> {
    if word.len() != ESCROW_PIN_LEN || word[0] != b'e' {
        return Err(NotEscrowPin);
    }
    let mut len = [0u8; 8];
    let mut pin = EscrowPin { chunk_root: [0; 32], content_len: 0, order_id: [0; 32] };
    decode(&word[1..65], &mut pin.chunk_root)?;
    decode(&word[65..81], &mut len)?;
    decode(&word[81..], &mut pin.order_id)?;
    pin.content_len = u64::from_be_bytes(len);
    Ok(pin)
}

/// `hex` into `out`, two lowercase digits a byte.
fn decode(hex: &[u8], out: &mut [u8]) -> Result<(), NotEscrowPin> {
    for (slot, pair) in out.iter_mut().zip(hex.chunks_exact(2)) {
        *slot = (digit(pair[0])? << 4) | digit(pair[1])?;
    }
    Ok(())
}

fn digit(c: u8) -> Result<u8, NotEscrowPin> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(NotEscrowPin),
    }
}
