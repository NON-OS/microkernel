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

//! What a claim refused for its isolation group says, pure: the
//! member that blocks it, the hop that lets the member reach it, and the
//! capsule that holds the member. kernel_proofs::acs_group.

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// Bus, device, function.
type Bdf = (u8, u8, u8);

pub(crate) fn refusal_line(member: Bdf, hop: Bdf, holder: &str) -> String {
    let (m, h) = (bdf(member), bdf(hop));
    format!("refused: {m} reaches it through {h} without ACS; {holder} holds it")
}

/// A requester-id alias of the device in another capsule's domain.
pub(crate) fn alias_line(alias: Bdf, holder: &str) -> String {
    format!("refused: its requester id {} is in the domain {holder} holds", bdf(alias))
}

/// A device of another capsule's whose DMA may carry the device's own
/// requester id (H-03, the alias check both ways).
pub(crate) fn peer_alias_line(peer: Bdf, holder: &str) -> String {
    format!("refused: {}, which {holder} holds, carries its requester id", bdf(peer))
}

/// A device of another capsule's whose isolation group cannot be read now:
/// the claim fails closed.
pub(crate) fn unreadable_line(peer: Bdf, holder: &str) -> String {
    format!("refused: the isolation group of {}, which {holder} holds, can't be read", bdf(peer))
}

fn bdf((b, d, f): Bdf) -> String {
    format!("{b:02x}:{d:02x}.{f}")
}
