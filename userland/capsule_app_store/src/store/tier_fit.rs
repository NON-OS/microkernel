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

//! Which Qwen tier cards this machine is shown, and in what order. A tier
//! is a card only when it fits this machine, by the rule setup's Qwen step
//! and the model fetcher hold a tier to (`need::tier_fits`): the person
//! chooses among tiers that will run, never one that cannot. The tiers that
//! do not fit are counted in one line instead of shown greyed, as setup
//! counts them. Where the model is kept decides the rule: on a live boot
//! the volume itself is memory (`Room::Memory`). Memory the kernel would
//! not report hides nothing, as the fetcher refuses nothing on it. Tiers
//! go smallest first, after every other listing in the market's order.
//! Pure, so model_fetch_proofs holds it over the real pins.

use alloc::format;
use alloc::string::String;

use crate::need::{tier_fits, Room};

/// The listings of shipped Qwen tiers are `linux.qwen-<tier>`.
pub const TIER_PREFIX: &[u8] = b"linux.qwen-";

/// The tier a listing is, if it is one.
pub fn tier_of(listing_id: &[u8]) -> Option<&str> {
    core::str::from_utf8(listing_id.strip_prefix(TIER_PREFIX)?).ok().filter(|t| !t.is_empty())
}

/// The summed length of `tier`'s files, from `weights`.
pub fn weight(tier: &str, weights: &[(&str, u64)]) -> Option<u64> {
    weights.iter().find(|(t, _)| *t == tier).map(|&(_, b)| b)
}

/// Whether a listing's card is shown on this machine: anything that is not
/// a tier, or a tier that fits.
pub fn shown(listing_id: &[u8], weights: &[(&str, u64)], memory: Option<u64>, room: Room) -> bool {
    let Some(tier) = tier_of(listing_id) else { return true };
    let (Some(total), Some(bytes)) = (memory, weight(tier, weights)) else { return true };
    tier_fits(room, tier, bytes, total) != Some(false)
}

/// Where a listing sorts: every other listing first, then tiers by size.
pub fn order(listing_id: &[u8], weights: &[(&str, u64)]) -> u64 {
    tier_of(listing_id).and_then(|t| weight(t, weights)).unwrap_or(0)
}

/// The line that counts the tiers not shown, when any are not.
pub fn hidden_line(n: usize) -> Option<String> {
    match n {
        0 => None,
        1 => Some(String::from("1 larger tier needs more memory than this machine has")),
        n => Some(format!("{n} larger tiers need more memory than this machine has")),
    }
}
