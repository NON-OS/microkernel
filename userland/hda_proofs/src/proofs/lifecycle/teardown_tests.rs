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

//! What the controller is still doing when its memory goes back.
//!
//! The broker scrubs and frees a DMA frame the moment the grant behind it
//! goes, and on a machine without an IOMMU nothing stops a ring engine that
//! is still running from writing into the frame after that. Every bring-up
//! that ends without playing (an HDMI-only controller, a codec that stops
//! answering) drops its broker handles with the CORB and RIRB engines on.
//! The shim records each broker call with the registers as they stood when
//! it was made, so the order is read off a log rather than inferred.

use nonos_libc::{logged, MK_PCI_CMD_BUS_MASTER};

use crate::constants::{CORBCTL_RUN, DPLBASE_ENABLE, GCTL_CRST, RIRBCTL_DMAEN, SDCTL_RUN};
use crate::proofs::lifecycle::teardown::{running, STREAMS};
use crate::proofs::lifecycle::teardown_log::{release, teardown, CMD, CORB, DPL, GCTL_AT, RIRB};

#[test]
fn every_engine_is_stopped_and_the_controller_reset_before_anything_is_given_back() {
    let log = teardown(&running());
    let r = &release(&log).regs;
    assert_eq!(r[CORB] & CORBCTL_RUN as u32, 0, "CORB engine running at release");
    assert_eq!(r[RIRB] & RIRBCTL_DMAEN as u32, 0, "RIRB engine running at release");
    assert_eq!(r[DPL] & DPLBASE_ENABLE, 0, "position buffer still written at release");
    for (s, ctl) in r[..STREAMS].iter().enumerate() {
        assert_eq!(ctl & SDCTL_RUN as u32, 0, "stream {s} running at release");
    }
    assert_eq!(r[GCTL_AT] & GCTL_CRST, 0, "controller out of reset at release");
    assert_eq!(r[CMD] as u16 & MK_PCI_CMD_BUS_MASTER, 0, "bus master still on at release");
    assert!(logged().is_empty(), "a clean teardown logged {:?}", logged());
}
