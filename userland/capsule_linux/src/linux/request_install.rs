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

//! An install request: the package, its pin and the market's word on its box.

use alloc::string::String;

use super::install::{market_arg, NotMarket, MARKET_LEN};
use super::request::args;

/// `install <name> <hash> <m> [direct]`: fetch a package rather than run a
/// program. The hash is the market's BLAKE3 of the package the user chose,
/// as hex; m is the release's policy and keys, or "-" (`install::market_arg`).
pub struct Install {
    pub name: String,
    pub pin: [u8; 32],
    /// Err when m is no word this capsule understands: the install fails.
    pub market: Result<Option<[u8; MARKET_LEN]>, NotMarket>,
}

pub fn install_request() -> Option<Install> {
    let args = args()?;
    let mut parts = args.split(|b| *b == 0);
    if parts.next()? != b"install" {
        return None;
    }
    let name = parts.next().filter(|s| !s.is_empty())?;
    let hex = parts.next().filter(|h| h.len() == 64)?;
    let mut pin = [0u8; 32];
    for (slot, pair) in pin.iter_mut().zip(hex.chunks(2)) {
        *slot = u8::from_str_radix(core::str::from_utf8(pair).ok()?, 16).ok()?;
    }
    let market = parts.next().map_or(Err(NotMarket), market_arg);
    Some(Install { name: String::from(core::str::from_utf8(name).ok()?), pin, market })
}

/// `install <name> <hash> <m> direct`: the person chose, in the store, to
/// download this Qwen tier's model over a direct connection, for this
/// install only (init's `linux_jobs::direct`).
pub fn direct_asked() -> bool {
    let Some(args) = args() else { return false };
    let mut parts = args.split(|b| *b == 0);
    parts.next() == Some(b"install") && parts.nth(3) == Some(b"direct")
}
