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
//! box, from the release (plan/30). init writes it, and only init can, since
//! the kernel's init sets a spawned process's arguments. "-" is a release
//! with no guest_policy; anything else is policy[256] | listing_key[32] |
//! release_key[32], then the release's listed arguments, if any, up to
//! nonos_linux_policy::args::MAX_LEN bytes, all as lowercase hex.

use alloc::vec::Vec;

use nonos_linux_policy::args::{check, MAX_LEN};

/// The policy and the two keys it must name.
pub const MARKET_LEN: usize = 256 + 32 + 32;

/// A fourth word that is neither: the install fails rather than drop the box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotMarket;

/// What the market sent: the policy with its keys, and the listed arguments,
/// empty for none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Market {
    pub head: [u8; MARKET_LEN],
    pub args: Vec<u8>,
}

impl Market {
    /// Its arguments, when there are none or they are canonical.
    pub fn listed(&self) -> Result<&[u8], &'static str> {
        match self.args.is_empty() || check(&self.args).is_ok() {
            true => Ok(&self.args),
            false => Err("its listed arguments are malformed"),
        }
    }
}

/// Ok(None) for "-", Ok(Some) for the 320 bytes and any arguments after.
pub fn market_arg(word: &[u8]) -> Result<Option<Market>, NotMarket> {
    if word == b"-" {
        return Ok(None);
    }
    let n = word.len();
    if !(MARKET_LEN * 2..=(MARKET_LEN + MAX_LEN) * 2).contains(&n) || !n.is_multiple_of(2) {
        return Err(NotMarket);
    }
    let bytes = word.chunks_exact(2).map(|pair| Ok((nibble(pair[0])? << 4) | nibble(pair[1])?));
    let bytes = bytes.collect::<Result<Vec<u8>, NotMarket>>()?;
    let (head, args) = bytes.split_at(MARKET_LEN);
    let head = head.try_into().map_err(|_| NotMarket)?;
    Ok(Some(Market { head, args: args.to_vec() }))
}

fn nibble(c: u8) -> Result<u8, NotMarket> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(NotMarket),
    }
}
