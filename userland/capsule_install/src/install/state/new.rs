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

//! The state an installer opens with: this boot's verdict and image, read
//! once, whether it runs from an installed NØNOS, and no disk chosen yet.

use alloc::vec::Vec;

use nonos_blk_client::{running_installed, Disk};

use super::choice::Choice;
use super::machine::State;
use super::screen::Screen;
use crate::install::source::{Boot, Image};

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
            prepared: None,
            job: None,
            outcome: None,
            notice,
            raid: false,
            incomplete: false,
            looked: None,
            keep: None,
            choice: Choice::Erase,
            installed: running_installed(),
            handover: None,
        }
    }

    pub fn selected_disk(&self) -> Option<&Disk> {
        self.disks.get(self.selected)
    }
}
