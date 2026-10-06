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

//! How large a buffer to offer and how to cut it.

use super::HmbAsk;

/// Pages of 4 KiB, the memory page size this driver enables the controller
/// with, which is also the unit every HMB field counts in.
pub const PAGE: u64 = 4096;
/// Most this driver gives without being asked for more as a minimum: 64 MiB.
const BUDGET_PAGES: u32 = 16_384;
/// Never more than 128 MiB, whatever the controller says it needs.
const CEILING_PAGES: u32 = 32_768;
/// One piece is one broker DMA map, at most the block class's 4 MiB.
pub const MAX_CHUNK_PAGES: u32 = 1024;
/// Descriptors fit one page of 16-byte entries.
pub const MAX_DESCRIPTORS: u32 = (PAGE / 16) as u32;

/// How the buffer is cut: `pieces` of `piece_pages` each.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HmbPlan {
    pub piece_pages: u32,
    pub pieces: u32,
}

impl HmbPlan {
    pub const fn pages(&self) -> u32 {
        self.piece_pages * self.pieces
    }
}

/// The buffer to offer, or `None` when the controller wants none or wants
/// more than this driver gives. Sized as Linux nvme_setup_host_mem sizes it:
/// the preferred size up to a budget (raised to the minimum when that is
/// larger, never past the ceiling), cut into the largest pieces one map can
/// hold, no more pieces than the controller takes.
pub fn plan(ask: HmbAsk) -> Option<HmbPlan> {
    if ask.preferred == 0 {
        return None;
    }
    if ask.minimum > CEILING_PAGES || ask.min_piece > MAX_CHUNK_PAGES {
        return None;
    }
    let budget = BUDGET_PAGES.max(ask.minimum);
    let want = ask.preferred.min(budget).max(ask.minimum);
    let max_pieces = match ask.max_pieces {
        0 => MAX_DESCRIPTORS,
        n => (n as u32).min(MAX_DESCRIPTORS),
    };
    let piece_pages = MAX_CHUNK_PAGES.min(want).max(ask.min_piece.max(1));
    let pieces = want.div_ceil(piece_pages).min(max_pieces);
    let plan = HmbPlan { piece_pages, pieces };
    if plan.pages() < ask.minimum {
        return None;
    }
    Some(plan)
}
