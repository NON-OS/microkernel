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

//! One claimed GPIO controller: its record, the register model its `_HID`
//! picks (`ctl`), and the router of its pins.

use crate::ctl::Ctl;
use alloc::vec::Vec;
use nonos_gpio::Router;
use nonos_libc::DeviceRecord;

pub struct Bank {
    pub record: DeviceRecord,
    pub uid: u16,
    pub ctl: Ctl,
    pub router: Router,
    /// `_AEI` pins armed level-triggered: masked while their method runs.
    pub level_aei: Vec<u16>,
    /// The controller's interrupt grant, 0 when its line is not bound.
    pub grant: u64,
    /// The grant's delivery count when the pins were last read.
    pub seq: u64,
}
