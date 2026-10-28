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

//! The state every screen reads and every event or tick changes.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_disk::Keep;

use super::choice::Choice;
use super::outcome::Outcome;
use super::prepared::Prepared;
use super::screen::Screen;
use crate::install::job::Job;
use crate::install::source::{Boot, Image};
use nonos_blk_client::Disk;

pub struct State {
    pub screen: Screen,
    pub disks: Vec<Disk>,
    pub selected: usize,
    /// What the person typed on the confirm screen. The install starts only
    /// when it equals the disk's label exactly.
    pub typed: Vec<u8>,
    pub image: Option<Image>,
    pub boot: Boot,
    /// The plan for the chosen disk, made when it was chosen, or why there
    /// is none.
    pub prepared: Option<Result<Prepared, String>>,
    pub job: Option<Job>,
    pub outcome: Option<Outcome>,
    /// Why the image or the disk list is unavailable, when it is.
    pub notice: Option<String>,
    /// The last look found no disk to install to and an Intel RST or VMD
    /// controller on the bus: the firmware's storage mode hides the disks.
    pub raid: bool,
    /// The last look found nothing to install to, or a missing driver, so
    /// the disks screen looks again on its own.
    pub incomplete: bool,
    /// When the last look at the disks ended and how long it took.
    pub looked: Option<Looked>,
    /// What an update would keep of the chosen disk, held to this image's
    /// boot files when it was chosen; `None` for one that holds no NONOS.
    pub keep: Option<Keep>,
    /// The confirm screen's row, on a disk an update could keep.
    pub choice: Choice,
    /// This boot runs from an installed NØNOS: the first screen offers to
    /// prepare it for an update, and says here what the kernel answered.
    pub installed: bool,
    pub handover: Option<i64>,
}

/// Milliseconds of uptime.
#[derive(Clone, Copy)]
pub struct Looked {
    pub ended: i64,
    pub took: i64,
}
