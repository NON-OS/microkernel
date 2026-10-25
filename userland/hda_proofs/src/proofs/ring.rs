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

//! Verbs that stop being answered (C1): two the controller never fetched
//! reset the rings in place and the verb goes once more; three resets with
//! no answer between them, or rings that will not stop, and the driver
//! gives the rings up for the Immediate Command interface. A verb fetched
//! and unanswered is a silent codec, and the rings are left alone.

mod fetched_tests;
mod give_up_tests;
mod immediate_tests;
mod reset_tests;
mod restart_tests;
mod rewedge_tests;
mod silent_tests;

use crate::constants::{CORBCTL, CORBCTL_RUN, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL};
use crate::controller::verb::Link;
use crate::regs::Regs;

const ANSWER: u32 = 0x1234_5678;
/// Get Parameter 0 of codec 0's root, and of codec 1's.
const TO_0: u32 = 0x000f_0000;
const TO_1: u32 = 0x100f_0000;

/// A link on rings the bring-up left running.
fn link_on(bar: &nonos_devmodel::FakeBar, corb: u64, rirb: u64) -> Link {
    bar.present8(CORBCTL as usize, CORBCTL_RUN);
    bar.present8(RIRBCTL as usize, RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    Link::new(Regs::new(bar.base()), corb, rirb, 256)
}
