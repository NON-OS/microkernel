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

//! The line under a Qwen tier's card: what installing it downloads and the
//! memory it needs to run. A tier is gigabytes fetched over the network and
//! held in memory while it answers, and a person should see both before
//! pressing Install, not after. The numbers are the pins' own lengths
//! (`model_weights.rs`) and the memory the model fetcher holds a download
//! to (`need.rs`), so the card, the fetcher's refusal and the signed
//! catalogue never disagree. The stick tier (`need::STICK_TIER`) is
//! carried by release sticks and imported with no network when this boot's
//! stick carries it, so its card says that rather than a download; whether
//! this stick does is the kernel's to say at the import, so the card names
//! release sticks, never this one. Pure, so model_fetch_proofs checks it
//! for every tier.

use alloc::format;
use alloc::vec::Vec;

use crate::need::{memory, STICK_TIER};
use crate::size::size;

use super::tier_fit::TIER_PREFIX;

/// "1.83 GB download, needs 2.41 GB of memory", for a Qwen tier's listing,
/// with `weights` each pinned tier and its files' summed length.
pub fn model_line(listing_id: &[u8], weights: &[(&str, u64)]) -> Option<Vec<u8>> {
    let tier = core::str::from_utf8(listing_id.strip_prefix(TIER_PREFIX)?).ok()?;
    let &(_, bytes) = weights.iter().find(|(t, _)| *t == tier)?;
    let need = memory(tier, bytes)?;
    let how = if tier == STICK_TIER { ", on release sticks;" } else { " download," };
    Some(format!("{}{how} needs {} of memory", size(bytes), size(need)).into_bytes())
}
