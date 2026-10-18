// NONOS Operating System (AGPL-3.0-or-later)
//! A whole HBA, the model port its port 0, plugged in for this thread's
//! register accesses until dropped: the controller a test opens.

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::block::Block;
use super::hba::Hba;
use super::hba_global::Global;
use super::plug::{plug, unplug};
use crate::constants::ata::STRUCT_REGION_BYTES;

pub struct HbaRig {
    pub hba: Rc<RefCell<Hba>>,
    /// The command list and FIS area port 0's engines were left running on.
    pub left: [Block; 2],
}

impl HbaRig {
    /// An HBA as a driver that died without its Drop left it: port 0's
    /// command list and FIS receive engines armed (ST and FRE set, CR and FR
    /// running), PxCLB and PxFB on frames of the dead driver's, which the
    /// kernel freed at the release, which no live grant holds (`iommu`). The
    /// broker shim reads it at every call.
    pub fn left_armed() -> Self {
        Self::with_ports(1)
    }

    /// As `left_armed`, on an HBA of `n` ports; ports 1 and up show their
    /// engines on as well.
    pub fn with_ports(n: u8) -> Self {
        let left = [Block::new(STRUCT_REGION_BYTES), Block::new(STRUCT_REGION_BYTES)];
        let mut hba = Hba::new(left[0].addr(), Vec::new());
        hba.ptr.fb = left[1].addr();
        hba.global = Some(Global::ports(n));
        let hba = Rc::new(RefCell::new(hba));
        plug(hba.clone());
        let watched = hba.clone();
        nonos_libc::watch(move || watched.borrow().engines());
        Self { hba, left }
    }

    pub fn hba(&self) -> RefMut<'_, Hba> {
        self.hba.borrow_mut()
    }
}

impl Drop for HbaRig {
    fn drop(&mut self) {
        unplug();
    }
}
