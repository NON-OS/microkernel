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

//! A reset controller holds nothing it was given (xHCI 1.2 section 5.4.1,
//! HCRST): followed, a reset forgets the command ring, every slot, a stuck
//! command and the errors USBSTS showed, and the event ring is found
//! again from the segment table once the driver sets RUN. Each is taken
//! before the reset or the start is shown done, so the driver never runs
//! ahead of the model.

use nonos_devmodel::FakeBar;

use super::hc::{Hc, SLOTS};
use crate::conformance::model::CAP_LEN;
use crate::constants::{USBCMD_HCRST, USBCMD_RUN, USBSTS, USBSTS_HCE, USBSTS_HSE};

const STS: usize = CAP_LEN + USBSTS as usize;

/// How the model takes an HCRST.
#[derive(Clone, Copy)]
pub enum Resets {
    /// As if it had not happened: the rings and `fault` stay.
    Ignored,
    /// Each one forgets everything.
    Followed,
    /// One forgot everything; the event ring is found again at RUN.
    Restarting,
}

impl Hc {
    /// Take a reset or a start the driver asked for in `cmd`, the USBCMD
    /// value `present` then shows done.
    pub(super) fn follow_reset(&mut self, bar: &FakeBar, cmd: u32) {
        match self.resets {
            Resets::Followed if cmd & USBCMD_HCRST != 0 => {
                (self.ring, self.stuck, self.fault) = (0, None, false);
                self.enabled = [false; SLOTS as usize + 1];
                bar.present32(STS, bar.wrote32(STS) & !(USBSTS_HSE | USBSTS_HCE));
                self.resets = Resets::Restarting;
            }
            Resets::Restarting if cmd & USBCMD_RUN != 0 => {
                self.producer.restart();
                self.resets = Resets::Followed;
            }
            _ => {}
        }
    }
}
