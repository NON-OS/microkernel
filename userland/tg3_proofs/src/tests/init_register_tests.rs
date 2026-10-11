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

//! The bring-up steps that need no answer from the part, on a window: every
//! vector masked, the 57765 fixups through the PCI Express window with
//! GRC_MODE put back, and host coalescing given tg3_init_coal's values.

use crate::constants::cpmu::*;
use crate::constants::grc::GRC_MODE;
use crate::constants::hostcc::*;
use crate::constants::mailbox::int_mbox;
use crate::init::{coalesce, fixups, power};

use super::build::{REV_57765_A0, REV_57766};
use super::model::{regs, window};

#[test]
fn every_vector_is_masked() {
    let bar = window();
    power::disable_ints(&regs(&bar));
    assert!((0..IRQ_MAX_VECS).all(|n| bar.wrote32(int_mbox(n)) == 1));
    assert_eq!(bar.wrote32(int_mbox(IRQ_MAX_VECS)), 0);
}

#[test]
fn fixups_reach_the_window_and_put_grc_mode_back() {
    for rev in [REV_57765_A0, REV_57766] {
        let bar = window();
        let r = regs(&bar);
        // SAFETY: both offsets are inside the window.
        unsafe {
            r.w32(GRC_MODE, 0x0012_0034);
            r.w32(TG3_PCIE_TLDLPL_PORT + TG3_PCIE_DL_LO_FTSMAX, 0x1234_56FF);
        }
        fixups::run(&r, rev);
        let phyctl5 = bar.wrote32(TG3_PCIE_TLDLPL_PORT + TG3_PCIE_PL_LO_PHYCTL5);
        assert_eq!(phyctl5 != 0, rev == REV_57765_A0, "{rev:#x}");
        assert_eq!(bar.wrote32(GRC_MODE), 0x0012_0034);
        assert_eq!(bar.wrote32(TG3_CPMU_PADRNG_CTL), TG3_CPMU_PADRNG_CTL_RDIV2);
        let fts = bar.wrote32(TG3_PCIE_TLDLPL_PORT + TG3_PCIE_DL_LO_FTSMAX);
        assert_eq!(fts, 0x1234_562C);
        assert_eq!(bar.wrote32(TG3_CPMU_LSPD_10MB_CLK), CPMU_LSPD_10MB_MACCLK_6_25);
    }
}

#[test]
fn coalescing_is_one_vector_at_tg3s_low_values() {
    let bar = window();
    coalesce::program(&regs(&bar));
    let r = |at| bar.wrote32(at);
    assert_eq!((r(HOSTCC_RXCOL_TICKS), r(HOSTCC_TXCOL_TICKS)), (0x32, 0x96));
    assert_eq!((r(HOSTCC_RXMAX_FRAMES), r(HOSTCC_TXMAX_FRAMES)), (0x05, 0x35));
    assert_eq!((r(HOSTCC_RXCOAL_MAXF_INT), r(HOSTCC_TXCOAL_MAXF_INT)), (0x05, 0x05));
    assert_eq!(r(HOSTCC_MODE), HOSTCC_MODE_ENABLE | HOSTCC_MODE_32BYTE);
}
