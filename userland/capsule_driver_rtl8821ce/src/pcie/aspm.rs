// NONOS Operating System (AGPL-3.0-or-later)

//! ASPM L1 and CLKREQ on the chip's side, as far as the host enabled them on
//! the link and only while the firmware sleeps (rtw88 `rtw_pci_link_cfg`,
//! `rtw_pci_link_ps`, `rtw_pci_aspm_set`). rtw88 sets the chip's CLKREQ
//! enable once at probe; here it follows L1, since a gated clock is what
//! bring-up found answering reads with zeros.

use super::awake::{BIT_CLKREQ_SW_EN, BIT_L1_SW_EN, RTK_PCIE_LINK_CFG};
use super::dbi;
use crate::regs::Mmio;

/// Link Control: ASPM L1 entry enabled, and clock power management
/// (PCI Express Base 6.0, 7.5.3.7).
pub const LNKCTL_ASPM_L1: u16 = 1 << 1;
pub const LNKCTL_CLKREQ_EN: u16 = 1 << 8;

/// What the host enabled on this link.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Aspm {
    pub l1: bool,
    pub clkreq: bool,
}

impl Aspm {
    /// From the host's Link Control register.
    pub fn from_link_control(lnkctl: u16) -> Self {
        Self { l1: lnkctl & LNKCTL_ASPM_L1 != 0, clkreq: lnkctl & LNKCTL_CLKREQ_EN != 0 }
    }

    /// The chip's own enables set entering power save, cleared leaving it,
    /// each only where the host enabled its half. Nothing is touched when the
    /// host enabled neither; false when a DBI access never completed.
    pub fn link_ps<M: Mmio>(self, mmio: &M, enter: bool) -> bool {
        let bits =
            if self.l1 { BIT_L1_SW_EN } else { 0 } | if self.clkreq { BIT_CLKREQ_SW_EN } else { 0 };
        if bits == 0 {
            return true;
        }
        let Some(v) = dbi::read8(mmio, RTK_PCIE_LINK_CFG) else { return false };
        let v = if enter { v | bits } else { v & !bits };
        dbi::write8(mmio, RTK_PCIE_LINK_CFG, v)
    }
}
