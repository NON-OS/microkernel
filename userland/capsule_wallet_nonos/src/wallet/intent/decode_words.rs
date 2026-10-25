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

/*
 * A publisher call's words read back, each only in the shape encode
 * writes: an address's top 12 bytes zero, a uint8 below 256, an amount in
 * u128 and above 0, one string at its offset with zero padding.
 */

use alloc::vec::Vec;

use nonos_nox_config::Contract;
use nonos_settle_proto::MAX_URL;

use super::abi::uint;

/// The three contracts an Approve may name as its spender.
pub(super) const SPENDERS: [Contract; 3] =
    [Contract::Settlement, Contract::Escrow, Contract::PublisherBond];

/// The first `N` words after `sel`, or None when the selector differs.
pub(super) fn heads<const N: usize>(data: &[u8], sel: u32) -> Option<[[u8; 32]; N]> {
    let (s, rest) = data.split_first_chunk::<4>()?;
    (u32::from_be_bytes(*s) == sel).then_some(())?;
    let (words, _) = rest.as_chunks::<32>();
    words.get(..N)?.try_into().ok()
}

/// A static call: exactly `N` words after `sel`.
pub(super) fn exact<const N: usize>(data: &[u8], sel: u32) -> Option<[[u8; 32]; N]> {
    (data.len() == 4 + 32 * N).then_some(())?;
    heads(data, sel)
}

fn zero(b: &[u8]) -> bool {
    b.iter().all(|x| *x == 0)
}

pub(super) fn address(w: &[u8; 32]) -> Option<[u8; 20]> {
    zero(&w[..12]).then(|| w[12..].try_into().ok()).flatten()
}

pub(super) fn uint8(w: &[u8; 32]) -> Option<u8> {
    zero(&w[..31]).then_some(w[31])
}

pub(super) fn amount(w: &[u8; 32]) -> Option<u128> {
    let v = u128::from_be_bytes(w[16..].try_into().ok()?);
    (zero(&w[..16]) && v > 0).then_some(v)
}

/// The string after `n` heads: offset, length (at most MAX_URL), bytes,
/// then zeros to the word's end and nothing after.
pub(super) fn string(data: &[u8], n: usize) -> Option<Vec<u8>> {
    let tail = data.get(4 + 32 * n..)?;
    (tail.get(..32)? == uint(32 * (n as u128 + 1))).then_some(())?;
    let len = usize::try_from(u128::from_be_bytes(tail.get(48..64)?.try_into().ok()?)).ok()?;
    let body = tail.get(64..)?;
    let whole = body.len() == len.div_ceil(32) * 32 && zero(body.get(len..)?);
    (zero(tail.get(32..48)?) && len <= MAX_URL && whole).then(|| body[..len].to_vec())
}
