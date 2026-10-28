// NONOS Operating System (AGPL-3.0-or-later)

//! What the layout and the boot are told: the adapter, and the regions.

use super::super::region::Region;

/// What the layout needs to know about the adapter.
pub struct Board {
    pub hw_rev: u32,
    pub imr_enabled: bool,
    /// CSR_HW_RF_ID, which picks the platform NVM section with the MAC type.
    pub rf_id: u32,
    /// The platform NVM file for this image, when the tree carries one.
    pub pnvm: Option<&'static [u8]>,
    /// The transport is not integrated (`iwl_pcie_set_ltr`, `start::cpu`).
    pub discrete: bool,
}

/// The DMA memory of one bring-up: the control region, the receive buffers,
/// and the firmware regions `firmware_region_sizes` asked for, in order.
pub struct Memory<'r, R: Region + ?Sized> {
    pub ctrl: &'r R,
    pub rbs: &'r R,
    pub fw: &'r [&'r R],
}
