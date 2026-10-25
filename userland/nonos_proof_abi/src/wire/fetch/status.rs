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

//! nonos.prove.fetch's statuses beyond the errnos, and a transfer's stages.

/// No relay is pinned yet (the owner's O10): nothing leaves the machine.
pub const NO_RELAY: i32 = -1201;
/// Not held yet: the download has started; ask STATUS.
pub const PENDING: i32 = -1202;
/// Route::chosen() is Direct or Down: fetch goes by Nym or Anyone or not at all.
pub const ROUTE: i32 = -1203;
/// The relay's bytes are not a container under this listing and anchor.
pub const ANCHOR: i32 = -1204;
/// The PUT's bytes are not NPRF v1 kind 1 with its exact sections.
pub const NOT_CONTAINER: i32 = -1205;
/// The relay did not answer within the budgets, or refused.
pub const RELAY: i32 = -1206;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Stage {
    Receiving = 0,
    Uploading = 1,
    Downloading = 2,
    /// The container is here: uploaded, or downloaded and checked.
    Held = 3,
    /// Ended without it; STATUS's status says why.
    Failed = 4,
}

impl Stage {
    pub fn from_u8(v: u8) -> Option<Stage> {
        [Stage::Receiving, Stage::Uploading, Stage::Downloading, Stage::Held, Stage::Failed]
            .get(usize::from(v))
            .copied()
    }
}

/// STATUS's reply body: the stage, then the container's length when known
/// (uploading or held), else 0.
pub fn status_body(stage: Stage, total: u32) -> [u8; 5] {
    let mut b = [0u8; 5];
    b[0] = stage as u8;
    b[1..].copy_from_slice(&total.to_le_bytes());
    b
}
