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

use super::outcome::Outcome;
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
    pub job: Option<Job>,
    pub outcome: Option<Outcome>,
    /// Why the image or the disk list is unavailable, when it is.
    pub notice: Option<String>,
}

impl State {
    pub fn new() -> Self {
        let boot = Boot::read();
        let (image, notice) = match Image::load() {
            Ok(image) => (Some(image), None),
            Err(why) => (None, Some(why)),
        };
        State {
            screen: Screen::Welcome,
            disks: Vec::new(),
            selected: 0,
            typed: Vec::new(),
            image,
            boot,
            job: None,
            outcome: None,
            notice,
        }
    }

    pub fn selected_disk(&self) -> Option<&Disk> {
        self.disks.get(self.selected)
    }
}
