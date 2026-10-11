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

//! The link as the join's radio.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> crate::assoc::Radio for RtlLink<M, D, RB> {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.send_raw(mpdu)
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<usize> {
        self.poll_raw(out)
    }
}
