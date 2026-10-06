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

use super::window::{window, Window, WindowError, PAGE_SIZE};

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn a_page_aligned_request_maps_as_it_always_did() {
    let w = window(0xFEBF_0000, 0x4000, 0x1000, 0x2000, &[]).unwrap();
    assert_eq!(
        w,
        Window { page_start: 0xFEBF_1000, page_bytes: 0x2000, in_page: 0, usable: 0x2000 }
    );
}

#[test]
fn an_intel_2k_abar_inside_a_page_maps_that_page() {
    // A PCH SATA ABAR: 2 KiB at a base ending in 0x800.
    let w = window(0xF7E3_6800, 0x800, 0, 0x800, &[]).unwrap();
    assert_eq!(
        w,
        Window { page_start: 0xF7E3_6000, page_bytes: PAGE_SIZE, in_page: 0x800, usable: 0x800 }
    );
    // Aligned at the start of its page, it maps the same one page.
    let w = window(0xF7E3_6000, 0x800, 0, 0x800, &[]).unwrap();
    assert_eq!((w.page_start, w.page_bytes, w.in_page, w.usable), (0xF7E3_6000, PAGE_SIZE, 0, 0x800));
}

#[test]
fn an_amd_1k_abar_off_a_page_boundary_maps_its_page() {
    // An AMD FCH SATA ABAR: 1 KiB, at a base that is 1 KiB aligned only.
    let w = window(0xFCE0_1400, 0x400, 0, 0x400, &[]).unwrap();
    assert_eq!(
        w,
        Window { page_start: 0xFCE0_1000, page_bytes: PAGE_SIZE, in_page: 0x400, usable: 0x400 }
    );
    // The last 1 KiB of a page.
    let w = window(0xFCE0_1C00, 0x400, 0, 0x400, &[]).unwrap();
    assert_eq!((w.page_start, w.page_bytes, w.in_page), (0xFCE0_1000, PAGE_SIZE, 0xC00));
}

#[test]
fn every_power_of_two_abar_from_one_port_up_maps_whole_at_any_aligned_base() {
    // Smallest useful ABAR: the globals and one port, 0x180, so 512 bytes up.
    for shift in 9..=16u32 {
        let size = 1u64 << shift;
        for slot in 0..(2 * PAGE_SIZE / size).max(1) {
            let base = 0xF000_0000 + slot * size;
            let w = window(base, size, 0, size, &[]).unwrap();
            assert_eq!(w.page_start + w.in_page, base, "{size:#x} at {base:#x}");
            assert_eq!(w.usable, size, "{size:#x} at {base:#x}");
            assert!(w.in_page + w.usable <= w.page_bytes, "{size:#x} at {base:#x}");
            assert_eq!(w.page_bytes, size.max(PAGE_SIZE), "{size:#x} at {base:#x}");
        }
    }
}

#[test]
fn a_sub_page_bar_crossing_a_page_boundary_maps_both_pages() {
    let w = window(0x1000_0C00, 0x800, 0, 0x800, &[]).unwrap();
    assert_eq!((w.page_start, w.page_bytes, w.in_page), (0x1000_0000, 2 * PAGE_SIZE, 0xC00));
}

#[test]
fn a_request_past_the_bar_is_refused() {
    assert_eq!(window(0xF7E3_6800, 0x800, 0, 0x801, &[]), Err(WindowError::BadRange));
    assert_eq!(window(0xF7E3_6800, 0x800, 0x700, 0x101, &[]), Err(WindowError::BadRange));
    assert_eq!(window(0xF7E3_6800, 0x800, 0, 0, &[]), Err(WindowError::ZeroLength));
    assert_eq!(window(u64::MAX - 8, 0x10, 0, 0x10, &[]), Err(WindowError::Overflow));
    assert_eq!(window(0, u64::MAX, u64::MAX - 8, 0x10, &[]), Err(WindowError::Overflow));
}

#[test]
fn an_msix_table_in_the_same_page_refuses_the_request() {
    // Another function's table in the other half of the ABAR's page.
    let table = (0xF7E3_6000, 0xF7E3_6100);
    assert_eq!(window(0xF7E3_6800, 0x800, 0, 0x800, &[table]), Err(WindowError::Protected));
    // After the request, still in its page.
    let table = (0xF7E3_6C00, 0xF7E3_6C10);
    assert_eq!(window(0xF7E3_6000, 0x800, 0, 0x800, &[table]), Err(WindowError::Protected));
}

#[test]
fn an_msix_table_in_a_later_page_cuts_the_mapping_short() {
    // xHCI-like: registers, then the table at 0x3000 into the BAR.
    let table = (0xFEB0_3000, 0xFEB0_3080);
    let pba = (0xFEB0_3800, 0xFEB0_3808);
    let w = window(0xFEB0_0000, 0x4000, 0, 0x4000, &[table, pba]).unwrap();
    assert_eq!((w.page_start, w.page_bytes, w.usable), (0xFEB0_0000, 0x3000, 0x3000));
    // Listed the other way round, the same cut.
    let w = window(0xFEB0_0000, 0x4000, 0, 0x4000, &[pba, table]).unwrap();
    assert_eq!(w.page_bytes, 0x3000);
}

#[test]
fn a_request_starting_in_a_protected_page_is_refused() {
    let table = (0xFEB0_3000, 0xFEB0_3080);
    assert_eq!(window(0xFEB0_0000, 0x4000, 0x3000, 0x1000, &[table]), Err(WindowError::Protected));
}

#[test]
fn an_empty_or_far_region_changes_nothing() {
    let far = (0x1_0000_0000, 0x1_0000_1000);
    let empty = (0xF7E3_6000, 0xF7E3_6000);
    let w = window(0xF7E3_6800, 0x800, 0, 0x800, &[far, empty]).unwrap();
    assert_eq!((w.page_bytes, w.usable), (PAGE_SIZE, 0x800));
}

/*
 * Whatever the BAR, request and regions, a granted window: maps whole pages
 * from a page boundary; starts its usable bytes at the requested first byte;
 * keeps them inside both the request and the mapped pages; and maps no page
 * holding a byte of any region.
 */
#[test]
fn no_window_maps_a_protected_page_or_strays_from_its_request() {
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    let mut granted = 0u32;
    for _ in 0..200_000 {
        let base = (xorshift(&mut s) % 0x10_0000) & !0x7f;
        let size = 0x80 + xorshift(&mut s) % 0x6000;
        let offset = xorshift(&mut s) % size;
        let length = 1 + xorshift(&mut s) % (size - offset + 0x40);
        let mut regions = Vec::new();
        for _ in 0..(xorshift(&mut s) % 4) {
            let lo = base.saturating_sub(0x2000) + xorshift(&mut s) % (size + 0x4000);
            regions.push((lo, lo + 1 + xorshift(&mut s) % 0x100));
        }
        let Ok(w) = window(base, size, offset, length, &regions) else { continue };
        granted += 1;
        let start = base + offset;
        assert_eq!(w.page_start % PAGE_SIZE, 0);
        assert_eq!(w.page_bytes % PAGE_SIZE, 0);
        assert!(w.page_bytes >= PAGE_SIZE);
        assert_eq!(w.page_start + w.in_page, start);
        assert!(w.usable >= 1 && w.usable <= length);
        assert!(w.in_page + w.usable <= w.page_bytes);
        assert!(start + w.usable <= base + size);
        let (lo, hi) = (w.page_start, w.page_start + w.page_bytes);
        for &(r_lo, r_hi) in &regions {
            assert!(r_hi <= lo || r_lo >= hi, "region {r_lo:#x}..{r_hi:#x} in {lo:#x}..{hi:#x}");
        }
        // No region in reach: the whole request is usable.
        if regions.is_empty() {
            assert_eq!(w.usable, length);
        }
    }
    assert!(granted > 10_000, "only {granted} windows granted");
}
