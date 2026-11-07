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

//! A device domain as the IOMMU keeps it: page by page, with the right each
//! page carries and a pending IOTLB the unmap must invalidate and wait on.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::rule::{DEVICE_READS, DEVICE_WRITES};
use super::runs::{Space, Step, PAGE};

#[derive(Default)]
pub struct Domain {
    /// IOVA page -> (physical page, readable, writable).
    pub pages: BTreeMap<u64, (u64, bool, bool)>,
    /// IOVA pages whose translation the IOTLB may still hold.
    pub cached: Vec<u64>,
    pub invalidations: usize,
    /// The map of the n-th run answers this (counted from 0).
    pub fail_map: Option<(usize, Step)>,
    /// The unit does not confirm unmaps.
    pub stuck: bool,
    pub maps: usize,
}

impl Domain {
    /// What the device can read at `iova`: a translation the tables or a
    /// stale IOTLB entry still give it.
    pub fn reaches(&self, iova: u64) -> bool {
        self.pages.contains_key(&iova) || self.cached.contains(&iova)
    }
}

impl Space for Domain {
    fn map(&mut self, iova: u64, phys: u64, len: u64) -> Step {
        let n = self.maps;
        self.maps += 1;
        let answer = match self.fail_map {
            Some((at, step)) if at == n => step,
            _ => Step::Mapped,
        };
        if matches!(answer, Step::Mapped | Step::Uncommitted | Step::Stranded) {
            for p in 0..len / PAGE {
                let at = iova + p * PAGE;
                assert!(!self.pages.contains_key(&at), "IOVA {at:#x} mapped twice");
                self.pages.insert(at, (phys + p * PAGE, DEVICE_READS, DEVICE_WRITES));
                self.cached.push(at);
            }
        }
        answer
    }

    fn unmap(&mut self, iova: u64, len: u64) -> bool {
        if self.stuck {
            return false;
        }
        let pages: Vec<u64> = (0..len / PAGE).map(|p| iova + p * PAGE).collect();
        if pages.iter().any(|p| !self.pages.contains_key(p)) {
            return false;
        }
        for p in &pages {
            self.pages.remove(p);
        }
        self.cached.retain(|c| !pages.contains(c));
        self.invalidations += 1;
        true
    }
}
