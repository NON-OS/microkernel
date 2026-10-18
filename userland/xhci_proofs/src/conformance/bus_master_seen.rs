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

//! What the controller showed at each broker call of a bring-up attempt,
//! one flag per bit of the log's word: whether it ran, was halted and ready,
//! had been reset, and had been given its DCBAA and its event ring segment
//! table, whose ERSTBA write starts the event ring (xHCI 1.2 5.5.2.3.2).

use nonos_devmodel::FakeBar;

use super::bringup_model::{CMD, ERSTBA, STS};
use super::model::{wrote64, CAP_LEN};
use crate::constants::{DCBAAP_LO, USBCMD_RUN, USBSTS_CNR, USBSTS_HCH};

pub const RUNNING: u64 = 1;
pub const HALTED: u64 = 1 << 1;
pub const NOT_READY: u64 = 1 << 2;
pub const RESET: u64 = 1 << 3;
pub const DCBAA_GIVEN: u64 = 1 << 4;
pub const ERST_GIVEN: u64 = 1 << 5;

/// The flags for `bar` now, the controller having taken `resets` HCRSTs.
pub fn seen(bar: &FakeBar, resets: u32) -> u64 {
    let (cmd, sts) = (bar.wrote32(CMD), bar.wrote32(STS));
    let flags = [
        (cmd & USBCMD_RUN != 0, RUNNING),
        (sts & USBSTS_HCH != 0, HALTED),
        (sts & USBSTS_CNR != 0, NOT_READY),
        (resets > 0, RESET),
        (wrote64(bar, CAP_LEN + DCBAAP_LO as usize) != 0, DCBAA_GIVEN),
        (wrote64(bar, ERSTBA) != 0, ERST_GIVEN),
    ];
    flags.iter().filter(|(on, _)| *on).fold(0, |word, (_, bit)| word | bit)
}
