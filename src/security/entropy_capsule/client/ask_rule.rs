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

//! When the kernel asks the entropy capsule for bytes, pure (K9). One ask
//! is out at a time; a reply counts only when it answers that ask, from the
//! capsule it went to; an ask unanswered for GIVE_UP_MS is given up, so a
//! lost reply never stops the asking. kernel_proofs::entropy_health.

pub(crate) const GIVE_UP_MS: u64 = 2_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Asked {
    pub(crate) id: u32,
    pub(crate) generation: u64,
    pub(crate) at_ms: u64,
}

/// Whether to send an ask now: the capsule is alive, bytes are wanted, and
/// no ask to this capsule is out and young.
pub(crate) fn due(
    asked: Option<Asked>,
    now_ms: u64,
    generation: u64,
    alive: bool,
    wanted: bool,
) -> bool {
    alive
        && wanted
        && asked.is_none_or(|a| {
            a.generation != generation || now_ms.saturating_sub(a.at_ms) >= GIVE_UP_MS
        })
}

/// Whether reply `id` from the capsule at `generation` answers the ask out.
pub(crate) fn answers(asked: Option<Asked>, id: u32, generation: u64) -> bool {
    asked.is_some_and(|a| a.id == id && a.generation == generation)
}
