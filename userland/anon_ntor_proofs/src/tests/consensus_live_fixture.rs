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
//! Where the live consensus comes from, when there is one.

use alloc::vec::Vec;

/*
 * The committed vectors are a trimmed slice, enough to pin field offsets and
 * malformed shapes. They cannot answer the question this file exists for: whether
 * the parsers survive five thousand relays written by five thousand operators,
 * and whether a path can still be drawn once the weights and the network rule are
 * both applied.
 *
 * A whole consensus is 1.8 MB and does not belong in the tree, so the document is
 * read from a path in the environment and the test passes quietly when there is
 * none. Fetch one with:
 *
 *   curl -s -o cons.z \
 *     http://49.13.145.234:9230/tor/status-vote/current/consensus-microdesc.z
 *
 * inflate it, and point ANON_LIVE_CONSENSUS at the result.
 */
pub(super) fn live() -> Option<Vec<u8>> {
    let path = std::env::var("ANON_LIVE_CONSENSUS").ok()?;
    std::fs::read(path).ok()
}

pub(super) fn weights_of(doc: &crate::directory::consensus::Consensus) -> Weights {
    doc.weights
}
