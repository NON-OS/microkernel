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

//! Which stream opening on the Nym mixnet takes its step in a pass of the
//! serve loop. Each step can hold the loop for a slice (nym.rs SLICE_MS)
//! while net.socks5 answers, so N streams opening at once held it N slices a
//! pass, and every other guest call in the family waited behind them. Now a
//! pass steps one opening stream at most, with the pass's one wait
//! (pass.rs), and that stream is the one that stepped longest ago, so every
//! opening stream gets its turn within N passes. Pure, so a host proof holds
//! it (capsule_linux_proofs, routed_opening_tests).

/// The stream whose turn it is among `opening`, each a stream's handle and
/// when it last stepped: the one that stepped longest ago, the lowest handle
/// among equals.
pub fn turn(opening: impl Iterator<Item = (u32, i64)>) -> Option<u32> {
    opening.min_by_key(|&(h, stepped)| (stepped, h)).map(|(h, _)| h)
}

/// Whether the stream `me` steps now: only on its turn, and only with the
/// pass's one wait, which `slot` takes.
pub fn may_step(me: u32, turn: Option<u32>, slot: impl FnOnce() -> bool) -> bool {
    turn == Some(me) && slot()
}
