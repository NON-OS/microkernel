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

//! The fourth word of an install request: the market's word on the program's
//! box, from the release. init writes it, and only init can, since
//! the kernel's init sets a spawned process's arguments. "-" is a release
//! with no guest_policy; anything else is policy[256] | listing_key[32] |
//! release_key[32] as 640 lowercase hex.

/// The policy and the two keys it must name.
pub const MARKET_LEN: usize = 256 + 32 + 32;

/// A fourth word that is neither: the install fails rather than drop the box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotMarket;

/// Ok(None) for "-", Ok(Some) for the 320 bytes.
pub fn market_arg(word: &[u8]) -> Result<Option<[u8; MARKET_LEN]>, NotMarket> {
    if word == b"-" {
        return Ok(None);
    }
    if word.len() != MARKET_LEN * 2 {
        return Err(NotMarket);
    }
    let mut out = [0u8; MARKET_LEN];
    for (slot, pair) in out.iter_mut().zip(word.chunks_exact(2)) {
        *slot = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Ok(Some(out))
}

fn nibble(c: u8) -> Result<u8, NotMarket> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(NotMarket),
    }
}
