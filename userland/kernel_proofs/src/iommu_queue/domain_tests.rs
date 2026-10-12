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

//! Invalidating one domain instead of every one, and only when a change
//! owes it: the descriptor and register encodings against VT-d 6.5.2, and
//! the rule a commit follows.

use super::descriptor::{iotlb_domain, iotlb_global};
use super::flush_rule::invalidates;

#[test]
fn a_domain_selective_descriptor_is_linuxs_dsi_flush() {
    // QI_IOTLB_TYPE 0x2, QI_IOTLB_GRAN(DMA_TLB_DSI_FLUSH) = 2 << 4,
    // QI_IOTLB_DID(did) = did << 16.
    assert_eq!(iotlb_domain(0x0123, false, false), [0x2 | 0x20 | (0x0123 << 16), 0]);
    assert_eq!(iotlb_domain(7, true, true)[0], 0x2 | 0x20 | 0x80 | 0x40 | (7 << 16));
    // Global stays global; the two differ only in granularity and the id.
    assert_eq!(iotlb_global(true, true)[0] & !0x30, iotlb_domain(0, true, true)[0] & !0x30);
}

#[test]
fn a_map_is_invalidated_only_under_caching_mode_and_an_unmap_always() {
    assert!(!invalidates(true, false), "an empty entry is never cached without CM");
    assert!(invalidates(true, true), "caching mode caches empty entries too");
    assert!(invalidates(false, false) && invalidates(false, true));
}

#[test]
fn the_register_path_names_the_domain_as_linux_does() {
    use crate::iommu_window::offsets::{IOTLB_DID_SHIFT, IOTLB_IIRG_DOMAIN, IOTLB_IIRG_GLOBAL};
    // DMA_TLB_GLOBAL_FLUSH 1 << 60, DMA_TLB_DSI_FLUSH 2 << 60, DMA_TLB_DID(id) id << 32.
    assert_eq!((IOTLB_IIRG_GLOBAL, IOTLB_IIRG_DOMAIN), (1 << 60, 2 << 60));
    assert_eq!(0x0123u64 << IOTLB_DID_SHIFT, 0x0123_0000_0000);
}
