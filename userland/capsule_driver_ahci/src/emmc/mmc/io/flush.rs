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

//! The cache flush: FLUSH_CACHE when the card's cache is on.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::Host;
use super::super::card::Card;
use super::super::ext_csd::FLUSH_CACHE;
use super::super::ops::{recover, say_failed, switch};

/// FLUSH_CACHE when the card's cache is on: every write it acknowledged is
/// then in its flash. With the cache off (or absent) every acknowledged
/// write already is, and there is nothing to do. The cache is left as the
/// firmware set it.
pub fn flush<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>, card: &Card) -> EmmcResult<()> {
    if !card.cache_on() {
        return Ok(());
    }
    if let Err(e) = switch(h, card, FLUSH_CACHE, 1) {
        say_failed(h, b"CMD6 FLUSH_CACHE", e);
        recover(h, card.rca);
        return Err(e);
    }
    Ok(())
}
