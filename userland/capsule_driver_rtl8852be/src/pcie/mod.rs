// NONOS Operating System (AGPL-3.0-or-later)

//! The PCIe glue: the PHY's MDIO and the config-space DBI windows, the
//! pre-init before the firmware download (`rtw89_pci_ops_mac_pre_init_ax`)
//! and the post-init after the MAC (`rtw89_pci_ops_mac_post_init_ax`).
//!
//! Power save is off by construction: rtw89 turns on the chip's own CLKREQ,
//! ASPM L1 and L1SS modules when the host enabled them (`pci.c`,
//! `rtw89_pci_link_cfg`, `rtw89_pci_l1ss_cfg`); this driver never calls
//! them, so those modules keep their reset state, disabled.

pub mod dbi;
mod dma;
pub mod mdio;
mod mode;
mod phy;
mod post;
pub mod regs;
pub mod regs_ltr;

pub use dma::{clear_idx, dma_all, dma_setup};
pub use mode::mode_op;
pub use phy::phy_setup;
pub use post::{ltr_bad, post_init};

/// The pre- or post-init step that did not complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieFail {
    Mdio,
    Dbi,
    /// The link rate read from config space is neither GEN1 nor GEN2.
    LinkRate(u8),
    TxBusy,
    RxBusy,
    BdRam,
    Ltr,
}

impl From<mdio::MdioTimeout> for PcieFail {
    fn from(_: mdio::MdioTimeout) -> Self {
        PcieFail::Mdio
    }
}

impl From<dbi::DbiTimeout> for PcieFail {
    fn from(_: dbi::DbiTimeout) -> Self {
        PcieFail::Dbi
    }
}
