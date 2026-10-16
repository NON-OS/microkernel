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

//! The controller made still before the memory it reads and writes goes back.
//!
//! The CORB, RIRB, BDL, PCM ring and position buffer are DMA grants, and the
//! broker scrubs and frees a frame as soon as its grant goes. Without an
//! IOMMU an engine still running then writes into whatever the frame has
//! become: an answer a codec gave after its deadline, a stream position. So
//! the teardown stops every engine, waits for each to read stopped, and puts
//! the controller in reset, before bus mastering is switched off and
//! anything is given back (plan rule C2).

use super::crst::enter_reset;
use super::stop::stop_engines;
use crate::constants::GCAP;
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

/// Stop every engine and hold the controller in reset. A failure is named,
/// but it does not end the teardown early: the reset is entered whether or
/// not every engine read stopped, since it stops them all. A controller that
/// reads all ones is not decoding and has nothing left to stop.
pub fn quiesce(regs: Regs) -> HdaResult<()> {
    // SAFETY: GCAP lies at offset 0 of every HDA register window.
    if unsafe { regs.r16(GCAP) } == 0xffff {
        return Err(HdaError::ControllerNotResponding);
    }
    let stopped = stop_engines(regs);
    let reset = enter_reset(regs);
    stopped.and(reset)
}
