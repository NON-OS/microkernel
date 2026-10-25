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

//! The parts of the takeover model's world (L79): a claim's stage as a
//! code, what a page of bootfb's mapping shows, and bootfb's claim entry.

use super::types::Stage;

/// A path done.
pub(super) const END: u8 = 100;

const STAGES: [Stage; 5] =
    [Stage::Making, Stage::Ready, Stage::Leaving, Stage::Orphaned, Stage::Exiting];

pub(super) fn code(s: Stage) -> u8 {
    STAGES.iter().position(|&t| t == s).unwrap_or(0) as u8
}

pub(super) fn stage(c: u8) -> Stage {
    STAGES[usize::from(c)]
}

/// What a page of bootfb's mapping shows. `Stale`: the sink in its tables,
/// while a CPU may still hold the framebuffer in its TLB until the round.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(super) enum Page {
    #[default]
    Nothing,
    Fb,
    Stale,
    Sink,
    Other,
}

impl Page {
    /// A store through the page may still land on the scanout.
    pub(super) fn shows_fb(self) -> bool {
        matches!(self, Page::Fb | Page::Stale)
    }
}

/// bootfb's claim: its epoch (its round plus one), stage code, the paths
/// inside it and the releases under way.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct Entry {
    pub epoch: u8,
    pub stage: u8,
    pub inflight: u8,
    pub leavers: u8,
}
