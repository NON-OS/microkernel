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

use crate::dma::DmaRegion;
use crate::log::{emit, Line};

/// The memory a controller was given, or may have taken. Only a disable
/// (CC.EN to 0) takes it back from the controller, so it goes back to the
/// broker with the device's release, after the disable and with bus
/// mastering off (setup::Device); only memory the controller never took is
/// unmapped before (`unmap`).
pub struct Hmb {
    pub(super) list: DmaRegion,
    pub(super) pieces: Vec<DmaRegion>,
    /// SET FEATURES dwords 11 to 15 that give it to the controller: at
    /// bring-up, and again after a reset has taken it back.
    pub enable: [u32; 5],
}

impl Hmb {
    /// Give back memory the controller never took: not offered, or refused.
    pub fn unmap(self) {
        for piece in self.pieces {
            piece.unmap();
        }
        self.list.unmap();
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
