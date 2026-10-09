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

use super::domain::*;
use super::sim::{Func, Sim};

const NVME: u32 = 0x5007_144d;
const MIB: u64 = 1 << 20;
const MEMBAR1: u64 = 0x7200_0000;

fn window() -> Window {
    pick_window((MEMBAR1, 32 * MIB), (0x60_3d10_0000, MIB)).unwrap()
}

fn bar64(bar: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (bar & !0xF) as u64
}

/// Shaped like a Tiger Lake VMD: root bus 224, two root ports, an NVMe
/// drive with a 16 KiB 64-bit BAR0 behind the first.
fn tiger_lake() -> Sim {
    let mut funcs = vec![Func::bridge(None, 6), Func::bridge(None, 0x1d)];
    let mut bars = [(0, false); 6];
    bars[0] = (0x4000, true);
    funcs.push(Func::endpoint(Some(0), 0, NVME, bars));
    Sim { root_bus: 224, funcs, writes_outside: 0 }
}

#[test]
fn vmd_ids_are_recognised_and_nothing_else() {
    for id in VMD_DEVICE_IDS {
        assert!(is_intel_vmd(0x8086, id), "{id:04x}");
        assert!(!is_intel_vmd(0x1022, id));
    }
    // Gemini Lake SATA and an RST SATA controller are not VMDs.
    assert!(!is_intel_vmd(0x8086, 0x31e3));
    assert!(!is_intel_vmd(0x8086, 0x282a));
}

#[test]
fn child_buses_start_where_vmconfig_says() {
    assert_eq!(bus_start(0x9a0b, 1, 0 << 8), Some(0));
    assert_eq!(bus_start(0x9a0b, 1, 1 << 8), Some(128));
    assert_eq!(bus_start(0x9a0b, 1, 2 << 8), Some(224));
    assert_eq!(bus_start(0x9a0b, 1, 3 << 8), None);
    // No restriction capability: bus 0, whatever VMCONFIG holds.
    assert_eq!(bus_start(0x9a0b, 0, 2 << 8), Some(0));
    // The Skylake server part has no such registers.
    assert_eq!(bus_start(0x201d, 1, 2 << 8), Some(0));
}

#[test]
fn cfgbar_holds_one_mib_per_bus_from_the_first() {
    assert_eq!(bus_count(224, 32 * MIB), 32);
    assert_eq!(bus_count(0, 256 * MIB), 256);
    assert_eq!(bus_count(240, 32 * MIB), 16);
    assert_eq!(cfg_offset(224, 32, 224, 0, 0, 0), Some(0));
    assert_eq!(cfg_offset(224, 32, 225, 0, 0, 0x10), Some(MIB + 0x10));
    assert_eq!(cfg_offset(224, 32, 224, 6, 0, 0), Some(6 << 15));
    assert_eq!(cfg_offset(224, 32, 224, 0, 3, 4), Some((3 << 12) + 4));
    assert_eq!(cfg_offset(224, 32, 223, 0, 0, 0), None);
    assert_eq!(cfg_offset(224, 16, 240, 0, 0, 0), None);
    assert_eq!(cfg_offset(224, 32, 224, 32, 0, 0), None);
    assert_eq!(cfg_offset(224, 32, 224, 0, 8, 0), None);
    assert_eq!(cfg_offset(224, 32, 224, 0, 0, 0x1000), None);
}

#[test]
fn the_window_prefers_memory_a_32_bit_bridge_window_reaches() {
    let w = pick_window((MEMBAR1, 32 * MIB), (0x60_3d10_0000, MIB)).unwrap();
    assert_eq!(w, Window { base: MEMBAR1, limit: MEMBAR1 + 32 * MIB - 1, prefetch64: false });
    // MEMBAR1 high, MEMBAR2 low: MEMBAR2 past the VMD's MSI-X pages.
    let w = pick_window((0x60_0000_0000, 32 * MIB), (0x7000_0000, 4 * MIB)).unwrap();
    assert_eq!(w.base, 0x7000_2000);
    assert!(!w.prefetch64);
    // Both high: the prefetchable 64-bit window.
    let w = pick_window((0x60_0000_0000, 32 * MIB), (0x61_0000_0000, MIB)).unwrap();
    assert!(w.prefetch64);
    assert_eq!(w.base, 0x60_0000_0000);
    assert_eq!(pick_window((0, 0), (0, 0)), None);
}

#[test]
fn the_drive_behind_a_root_port_gets_a_bus_and_its_bar() {
    let mut sim = tiger_lake();
    let w = window();
    let done = assign(&mut sim, 224, 32, w);
    assert_eq!(done.bridges, 2);
    assert_eq!(done.endpoints, 1);
    assert_eq!(done.bars, 1);
    assert_eq!(done.starved, 0);
    assert_eq!(done.last_bus, 226);
    assert_eq!(sim.writes_outside, 0);

    let port = sim.at(224, 6, 0).unwrap();
    let buses = port.reg(0x18);
    assert_eq!(buses & 0xFF, 224);
    assert_eq!((buses >> 8) & 0xFF, 225);
    assert_eq!((buses >> 16) & 0xFF, 225);
    assert_eq!(port.reg(0x04) & 0x6, 0x6, "port decodes memory and forwards DMA");

    let nvme = sim.at(225, 0, 0).expect("the drive answers on the port's secondary bus");
    let base = bar64(nvme.reg(0x10), nvme.reg(0x14));
    assert!(base >= w.base && base + 0x4000 - 1 <= w.limit);
    assert_eq!(base % 0x4000, 0);
    assert_eq!(nvme.reg(0x04) & 0x7, 0x2, "memory on, mastering left to the driver");

    let mem = port.reg(0x20);
    let open = ((mem & 0xFFF0) as u64) << 16;
    let close = (((mem >> 16) & 0xFFF0) as u64) << 16 | 0xF_FFFF;
    assert!(open <= base && base + 0x4000 - 1 <= close, "the port forwards the BAR");
    assert_eq!(port.reg(0x24), 0x0000_FFF0, "prefetchable window closed");

    let empty = sim.at(224, 0x1d, 0).unwrap();
    assert_eq!((empty.reg(0x18) >> 8) & 0xFF, 226);
    assert_eq!(empty.reg(0x20), 0x0000_FFF0, "nothing behind it, nothing forwarded");
}

#[test]
fn firmware_numbers_are_replaced_not_trusted() {
    let mut sim = tiger_lake();
    // RST left the port pointing at a bus outside the domain.
    sim.funcs[0].regs[6] = 0x00_09_09_00;
    let done = assign(&mut sim, 224, 32, window());
    assert_eq!(done.endpoints, 1);
    assert!(sim.at(225, 0, 0).is_some());
}

#[test]
fn a_window_too_small_leaves_bars_unassigned_not_overlapping() {
    let mut sim = tiger_lake();
    let mut bars = [(0, false); 6];
    bars[0] = (4 * MIB, true);
    sim.funcs.push(Func::endpoint(Some(0), 1, NVME, bars));
    let tight = Window { base: MEMBAR1, limit: MEMBAR1 + 2 * MIB - 1, prefetch64: false };
    let done = assign(&mut sim, 224, 32, tight);
    assert_eq!(done.bars, 1);
    assert_eq!(done.starved, 1);
    let big = sim.at(225, 1, 0).unwrap();
    assert_eq!(bar64(big.reg(0x10), big.reg(0x14)), 0);
    assert_eq!(big.reg(0x04) & 0x4, 0);
}

#[test]
fn running_out_of_buses_starves_the_bridge_not_the_walk() {
    let mut sim = tiger_lake();
    let done = assign(&mut sim, 224, 2, window());
    assert_eq!(done.bridges, 1);
    assert_eq!(done.starved, 1);
    assert_eq!(done.last_bus, 225);
    assert!(sim.at(225, 0, 0).is_some());
}

#[test]
fn a_32_bit_bar_is_not_placed_above_4_gib() {
    let mut sim = tiger_lake();
    let mut bars = [(0, false); 6];
    bars[0] = (0x4000, false);
    sim.funcs[2] = Func::endpoint(Some(0), 0, NVME, bars);
    let high = pick_window((0x60_0000_0000, 32 * MIB), (0x61_0000_0000, MIB)).unwrap();
    let done = assign(&mut sim, 224, 32, high);
    assert_eq!(done.starved, 1);
    assert_eq!(done.bars, 0);
}

#[test]
fn a_high_window_is_routed_through_the_prefetchable_registers() {
    let mut sim = tiger_lake();
    let high = pick_window((0x60_0000_0000, 32 * MIB), (0x61_0000_0000, MIB)).unwrap();
    assign(&mut sim, 224, 32, high);
    let port = sim.at(224, 6, 0).unwrap();
    assert_eq!(port.reg(0x20), 0x0000_FFF0);
    assert_eq!(port.reg(0x28), 0x60);
    assert_eq!(port.reg(0x2C), 0x60);
    assert_eq!(port.reg(0x24) & 0xF, 1);
    let nvme = sim.at(225, 0, 0).unwrap();
    assert_eq!(bar64(nvme.reg(0x10), nvme.reg(0x14)), 0x60_0000_0000);
}

#[test]
fn the_pci_layer_and_the_inventory_name_the_same_vmds() {
    let mut pci = VMD_DEVICE_IDS.to_vec();
    let mut inventory = super::inventory_vmd::INTEL_VMD_DEVICE_IDS.to_vec();
    pci.sort_unstable();
    inventory.sort_unstable();
    assert_eq!(pci, inventory);
    for id in 0..=u16::MAX {
        assert_eq!(is_intel_vmd(0x8086, id), super::inventory_vmd::is_intel_vmd(0x8086, id));
    }
}
