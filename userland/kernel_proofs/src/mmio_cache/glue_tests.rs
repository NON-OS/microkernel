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

//! The rule's place in the map path, read from the source: asked before
//! anything is mapped, its answer carried into the page table, and a
//! write-combined page never also marked a device register.

const MAP: &str = include_str!("../../../../src/hardware/broker/mmio/map.rs");
const INSTALL: &str = include_str!("../../../../src/hardware/broker/mmio/map_install.rs");
const PAGES: &str =
    include_str!("../../../../src/memory/paging/manager/api/mapping/map_user_mmio.rs");
const BGA: &str = include_str!("../../../capsule_driver_bga/src/setup/sequence.rs");
const LIBC: &str = include_str!("../../../libc/src/broker/mmio.rs");
const BOOTFB: &str = include_str!("../../../capsule_driver_bootfb/src/libc_firmware.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing: {needle}"))
}

#[test]
fn map_asks_the_rule_before_it_maps_and_carries_its_answer() {
    let asked =
        at(MAP, "cache::cache_for(req.flags, bar_held, (w.page_start, w.page_bytes), wc_ready)");
    let mapped = at(MAP, "let user_va = install(pid, &w, kind)?;");
    assert!(asked < mapped);
    assert!(MAP.contains("let wc = cache == Cache::WriteCombined;"));
    assert!(MAP.contains("let kind = memory_type(if wc { MMIO_MAP_WC } else { 0 }, &bar, &w)?;"));
    assert!(INSTALL.contains("let wc = kind == MemType::WriteCombining;"));
    assert!(INSTALL.contains("map_user_mmio(user_va, phys, size, wc)"));
    assert!(MAP.contains("if !cache::known(req.flags) {"));
}

#[test]
fn a_write_combined_page_is_pwt_alone_and_only_once_the_pat_is_ready() {
    let wc = "if write_combining && crate::arch::write_combining::write_combining_ready() {\n        base | PagePermissions::WRITE_THROUGH\n    } else {";
    assert!(PAGES.contains(wc));
    let uc = "base | PagePermissions::NO_CACHE | PagePermissions::WRITE_THROUGH | PagePermissions::DEVICE";
    assert!(PAGES.contains(uc));
    assert!(!PAGES.contains("let base = PagePermissions::USER | PagePermissions::READ | PagePermissions::WRITE | PagePermissions::DEVICE"));
}

#[test]
fn the_drivers_send_the_bit_the_kernel_knows() {
    assert_eq!(super::cache::MAP_WC, 1 << 0);
    assert!(LIBC.contains("pub const MK_MMIO_MAP_WC: u32 = 1 << 0;"));
    assert!(BGA.contains("use nonos_libc::{mk_device_release, MK_MMIO_MAP_WC};"));
    assert!(BOOTFB.contains("const MK_MMIO_MAP_WC: u32 = 1 << 0;"));
}
