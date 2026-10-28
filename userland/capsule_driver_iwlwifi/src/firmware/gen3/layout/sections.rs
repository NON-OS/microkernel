// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware sections copied into their regions, as Linux v6.12
//! `iwl_pcie_init_fw_sec` places them for either context info: each in its
//! own block, LMAC, UMAC and paging in that order.

use alloc::vec::Vec;

use super::super::plan::{plan_firmware, Block};
use super::super::region::Region;
use super::super::FwLayout;
use super::{LayoutError, Memory};

/// The sections' device addresses: LMAC, UMAC and paging in that order.
pub struct Placed {
    pub addrs: Vec<u64>,
    pub n_lmac: usize,
    pub n_umac: usize,
}

impl Placed {
    /// The three image maps: UMAC, LMAC and paging.
    pub fn maps(&self) -> (&[u64], &[u64], &[u64]) {
        let (lmac, rest) = self.addrs.split_at(self.n_lmac);
        let (umac, virt) = rest.split_at(self.n_umac);
        (umac, lmac, virt)
    }
}

pub(super) fn section_lens(fw: &FwLayout<'_>) -> Vec<usize> {
    fw.lmac.iter().chain(fw.umac.iter()).chain(fw.virt.iter()).map(|s| s.data.len()).collect()
}

/// Copy every section into the firmware regions `firmware_region_sizes`
/// asked for, and say where each went.
pub fn place<R: Region + ?Sized>(
    mem: &Memory<'_, R>,
    fw: &FwLayout<'_>,
) -> Result<Placed, LayoutError> {
    let fw_regions = mem.fw;
    let plan = plan_firmware(&section_lens(fw)).ok_or(LayoutError::Plan)?;
    if plan.regions.len() != fw_regions.len()
        || plan.regions.iter().zip(fw_regions).any(|(&want, r)| r.len() < want)
    {
        return Err(LayoutError::Plan);
    }
    let sections = fw.lmac.iter().chain(fw.umac.iter()).chain(fw.virt.iter());
    let mut addrs = Vec::with_capacity(plan.blocks.len());
    for (s, b) in sections.zip(&plan.blocks) {
        let Block { region, off, .. } = *b;
        let r = fw_regions[region];
        if !r.write(off, s.data) {
            return Err(LayoutError::Write);
        }
        addrs.push(r.dev() + off as u64);
    }
    Ok(Placed { addrs, n_lmac: fw.lmac.len(), n_umac: fw.umac.len() })
}
