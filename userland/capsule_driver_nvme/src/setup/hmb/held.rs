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

use alloc::vec::Vec;

use crate::admin::reset_to_disabled;
use crate::controller::ControllerInfo;
use crate::dma::DmaRegion;
use crate::log::{emit, Line};
use crate::regs::Regs;

/// The memory a controller was given, or may have taken. Only a disable
/// (CC.EN to 0) takes a host memory buffer back from the controller, so the
/// memory is unmapped only once one has completed.
pub struct Hmb {
    pub(super) list: DmaRegion,
    pub(super) pieces: Vec<DmaRegion>,
}

impl Hmb {
    /// Disable the controller, then let the memory go.
    pub fn release(self, regs: Regs, info: ControllerInfo) {
        let disabled = reset_to_disabled(regs, info).is_ok();
        self.after_disable(disabled);
    }

    /// Unmap the memory if the controller was disabled. If the disable
    /// failed the controller may still write to it, so it is not unmapped
    /// here: the grants stay until the device's release, which turns bus
    /// mastering off before the broker frees them.
    pub fn after_disable(self, disabled: bool) {
        if disabled {
            drop(self.pieces);
            drop(self.list);
            return;
        }
        let kept = self.pieces.len() as u64;
        say(b"disable failed with host memory buffer given; kept mapped, pieces ", kept);
        core::mem::forget(self);
    }
}

pub(super) fn say(what: &[u8], n: u64) {
    let mut line = Line::new();
    line.text(what);
    if n != 0 {
        line.dec(n);
    }
    emit(&mut line);
}
