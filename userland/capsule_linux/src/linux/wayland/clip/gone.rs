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

//! A source or an offer the client destroyed. Pure, so the host proofs
//! hold it.

use alloc::vec::Vec;

use super::offer::offer_all;
use super::state::Clip;
use super::which::Which;
use super::Objects;

/// The source `id` is destroyed: when it was the selection there is none,
/// and `fallback` is offered in its place.
pub fn source_gone(
    objects: &mut Objects,
    clip: &mut Clip,
    (which, id): (Which, u32),
    fallback: Option<u32>,
    out: &mut Vec<u8>,
) {
    clip.sources.retain(|s| s.0 != id);
    if clip.held[which.at()] == Some(id) {
        clip.held[which.at()] = None;
        offer_all(objects, clip, which, fallback, out);
    }
}

/// The client destroyed the offer `id`. A server id is the server's to
/// free, so no delete_id is sent for it, as libwayland-server sends none.
pub fn offer_gone(objects: &mut Objects, clip: &mut Clip, id: u32) {
    clip.offers.retain(|o| o.0 != id);
    objects.drop_id(id);
}
