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

//! Whether a namespace gets an I/O queue, and how much one command may move,
//! is decided from the identify pages, which the controller writes. The
//! proofs build those pages byte by byte and run the driver's real parsers
//! and its real decision over them: an absent or empty namespace, a block
//! size the buffers were not built for, a format with metadata or an index
//! the namespace does not have never get a queue, and no MDTS lets a
//! command move more than the data buffer holds.

use crate::admin::{ControllerIdentity, NamespaceIdentity};
use crate::nvm::{NamespaceGeometry, MAX_SECTORS, SECTOR_SIZE};
use crate::server::handlers::parse_rw_within;

/// The driver's data buffer: MAX_SECTORS sectors of SECTOR_SIZE bytes.
pub(crate) const DATA_BYTES: u64 = MAX_SECTORS as u64 * SECTOR_SIZE as u64;

/// An identify controller page whose MDTS is `mdts`; the rest is zero.
pub(crate) fn controller_with_mdts(mdts: u8) -> ControllerIdentity {
    let mut page = [0u8; 4096];
    page[0x4d] = mdts;
    ControllerIdentity::parse(&page)
}

/// An identify namespace page: NSZE, NLBAF (zero-based), FLBAS, and the
/// LBA format slots as (metadata bytes, LBADS) pairs.
pub(crate) fn ns_page(nsze: u64, nlbaf: u8, flbas: u8, formats: &[(u16, u8)]) -> [u8; 4096] {
    let mut page = [0u8; 4096];
    page[0x00..0x08].copy_from_slice(&nsze.to_le_bytes());
    page[0x08..0x10].copy_from_slice(&nsze.to_le_bytes());
    page[0x19] = nlbaf;
    page[0x1a] = flbas;
    for (i, &(ms, lbads)) in formats.iter().enumerate() {
        let slot = 0x80 + i * 4;
        page[slot..slot + 2].copy_from_slice(&ms.to_le_bytes());
        page[slot + 2] = lbads;
    }
    page
}

pub(crate) fn accept_page(page: &[u8; 4096]) -> Option<NamespaceGeometry> {
    NamespaceGeometry::check(&controller_with_mdts(0), &NamespaceIdentity::parse(1, page)).ok()
}

#[test]
fn a_namespace_the_controller_did_not_report_gets_no_queue() {
    let id = controller_with_mdts(0);
    // NN = 0: bring-up does not identify a namespace and records it absent.
    assert!(NamespaceGeometry::check(&id, &NamespaceIdentity::absent()).ok().is_none());
    // NSID 0 names no namespace, whatever page comes with it.
    let page = ns_page(1 << 20, 0, 0, &[(0, 9)]);
    assert!(NamespaceGeometry::check(&id, &NamespaceIdentity::parse(0, &page)).ok().is_none());
}

#[test]
fn an_empty_namespace_gets_no_queue() {
    // NSZE = 0 is also what an inactive NSID 1 identifies as.
    for lbads in [9, 12] {
        assert!(accept_page(&ns_page(0, 0, 0, &[(0, lbads)])).is_none(), "LBADS {lbads}");
    }
    assert!(accept_page(&[0u8; 4096]).is_none());
}

#[test]
fn only_the_block_sizes_the_buffers_were_built_for_are_taken() {
    for lbads in 0..=255u8 {
        let g = accept_page(&ns_page(1 << 20, 0, 0, &[(0, lbads)]));
        let expect = matches!(lbads, 9 | 12);
        assert_eq!(g.is_some(), expect, "LBADS {lbads}");
        if let Some(g) = g {
            assert_eq!(g.lba_size, 1 << lbads);
        }
    }
}

#[test]
fn a_format_carrying_metadata_is_refused() {
    for lbads in [9u8, 12] {
        for ms in [1u16, 8, 16, 64, 0x8000, 0xffff] {
            // FLBAS bit 4 set puts the metadata inside each block, clear in a
            // separate buffer; the driver provides for neither.
            for flbas in [0x00u8, 0x10] {
                let g = accept_page(&ns_page(1 << 20, 0, flbas, &[(ms, lbads)]));
                assert!(g.is_none(), "LBADS {lbads} with {ms} metadata bytes, FLBAS {flbas:#x}");
            }
        }
        assert!(accept_page(&ns_page(1 << 20, 0, 0x10, &[(0, lbads)])).is_some());
    }
}

#[test]
fn a_format_index_past_nlbaf_is_refused() {
    // Every slot holds a usable 512-byte format, so only the index decides.
    let formats = [(0u16, 9u8); 16];
    for nlbaf in 0..16u8 {
        for index in 0..16u8 {
            let g = accept_page(&ns_page(1 << 20, nlbaf, index, &formats));
            assert_eq!(g.is_some(), index <= nlbaf, "FLBAS {index} with NLBAF {nlbaf}");
        }
    }
}

#[test]
fn an_index_reaching_past_the_sixteen_slots_is_refused() {
    let formats = [(0u16, 12u8); 16];
    for upper in 1..4u8 {
        for index in 0..16u8 {
            let flbas = (upper << 5) | index;
            let page = ns_page(1 << 20, 63, flbas, &formats);
            assert_eq!(NamespaceIdentity::parse(1, &page).format_index_upper, upper);
            assert!(accept_page(&page).is_none(), "FLBAS {flbas:#x}");
        }
    }
}

#[test]
fn an_accepted_geometry_is_the_page_the_controller_wrote() {
    for (nsze, lbads) in [(1u64, 9u8), (8, 12), (u64::MAX, 9), (0x1_0000_0000, 12)] {
        let page = ns_page(nsze, 0, 0, &[(0, lbads)]);
        let id = controller_with_mdts(0);
        let g = NamespaceGeometry::check(&id, &NamespaceIdentity::parse(7, &page)).ok();
        let g = g.expect("a plain namespace is taken");
        assert_eq!((g.nsid, g.capacity_sectors, g.lba_size), (7, nsze, 1 << lbads));
    }
}

/// What the spec lets one command move under this MDTS (4 KiB pages), or
/// None for no limit, worked out apart from the driver.
fn spec_mdts_bytes(mdts: u8) -> Option<u128> {
    if mdts == 0 {
        None
    } else {
        Some(4096u128 << mdts.min(100))
    }
}

#[test]
fn no_mdts_takes_a_command_past_the_data_buffer() {
    for mdts in 0..=255u8 {
        for lbads in [9u8, 12] {
            let page = ns_page(1 << 30, 0, 0, &[(0, lbads)]);
            let ns = NamespaceIdentity::parse(1, &page);
            let g = NamespaceGeometry::check(&controller_with_mdts(mdts), &ns)
                .expect("MDTS alone never refuses a namespace");
            let lba = 1u64 << lbads;
            let bytes = g.max_sectors as u64 * lba;
            assert!(g.max_sectors >= 1, "MDTS {mdts}");
            assert!(bytes <= DATA_BYTES, "MDTS {mdts} lets {bytes} bytes through");
            let allowed =
                spec_mdts_bytes(mdts).map_or(DATA_BYTES, |m| m.min(DATA_BYTES as u128) as u64);
            assert_eq!(g.max_sectors as u64, allowed / lba, "MDTS {mdts}, LBADS {lbads}");
            // The server bounds every request by this ceiling.
            for n in [1, g.max_sectors, g.max_sectors + 1, MAX_SECTORS, u32::MAX] {
                let mut body = [0u8; 12];
                body[8..12].copy_from_slice(&n.to_le_bytes());
                if let Ok((_, taken)) = parse_rw_within(&body, 1 << 30, g.max_sectors) {
                    assert!(taken as u64 * lba <= DATA_BYTES.min(allowed));
                }
            }
        }
    }
}

#[test]
fn a_small_mdts_shrinks_each_command() {
    // (MDTS, sectors per command at 512-byte LBAs, at 4096-byte LBAs)
    for (mdts, at_512, at_4096) in
        [(0u8, 64u32, 8u32), (1, 16, 2), (2, 32, 4), (3, 64, 8), (255, 64, 8)]
    {
        for (lbads, want) in [(9u8, at_512), (12, at_4096)] {
            let page = ns_page(1 << 30, 0, 0, &[(0, lbads)]);
            let ns = NamespaceIdentity::parse(1, &page);
            let g = NamespaceGeometry::check(&controller_with_mdts(mdts), &ns).ok();
            assert_eq!(g.map(|g| g.max_sectors), Some(want), "MDTS {mdts}, LBADS {lbads}");
        }
    }
}
