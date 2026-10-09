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

//! The platform NVM step: finding this adapter's section in a PNVM file as
//! Linux's `iwl_pnvm_parse` does, laying it out as fragmented payloads with a
//! descriptor array (`iwl_pcie_load_payloads_segments`), pointing the
//! peripheral scratch at it, and the boot giving it to the firmware before the
//! doorbell when ALIVE names a SKU. The files are built here in the layout of
//! linux-firmware's iwlwifi-*.pnvm (SKU, version, HW type, two SEC_RT chunks).

use std::rc::Rc;

use crate::gen3::bringup::boot;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len_pnvm, pnvm_off, PAGE, PRPH_SCRATCH, RB_REGION};
use crate::gen3::pnvm::{
    area_len, lay_out, parse, point_scratch, reserve, Placed, DESC_LEN, MAX_CHUNKS,
};
use crate::gen3::region::Region;
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

static SO_GF: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

const MAC_SO: u16 = 0x37;
const MAC_SOF: u16 = 0x43;
const RF_GF: u16 = 0x10D;

fn tlv(out: &mut Vec<u8>, kind: u32, value: &[u8]) {
    out.extend_from_slice(&kind.to_le_bytes());
    out.extend_from_slice(&(value.len() as u32).to_le_bytes());
    out.extend_from_slice(value);
    while !out.len().is_multiple_of(4) {
        out.push(0);
    }
}

struct Section {
    sku: [u32; 3],
    hw: Vec<(u16, u16)>,
    chunks: Vec<Vec<u8>>,
    version: u32,
}

fn file(sections: &[Section]) -> Vec<u8> {
    let mut out = Vec::new();
    for s in sections {
        let sku: Vec<u8> = s.sku.iter().flat_map(|w| w.to_le_bytes()).collect();
        tlv(&mut out, 64, &sku);
        tlv(&mut out, 62, &s.version.to_le_bytes());
        for &(m, r) in &s.hw {
            let mut v = m.to_le_bytes().to_vec();
            v.extend_from_slice(&r.to_le_bytes());
            tlv(&mut out, 58, &v);
        }
        for (i, c) in s.chunks.iter().enumerate() {
            let mut v = (i as u32 * 0x1000).to_le_bytes().to_vec();
            v.extend_from_slice(c);
            tlv(&mut out, 19, &v);
        }
    }
    out
}

/* Shaped like iwlwifi-so-a0-gf-a0.pnvm: four SKUs, each for SO and SO-F with a
GF radio, each two chunks of about 6.8 KB. */
fn so_file() -> Vec<u8> {
    let s = |sku: [u32; 3], fill: u8, version: u32| Section {
        sku,
        hw: vec![(MAC_SO, RF_GF), (MAC_SOF, RF_GF)],
        chunks: vec![vec![fill; 6830], vec![fill ^ 0xFF; 6826]],
        version,
    };
    file(&[
        s([0x0006_10D1, 0, 0], 0x11, 0xAAAA_0001),
        s([0x0006_10D1, 3, 0], 0x22, 0xAAAA_0002),
        s([0x000A_10D1, 0, 0], 0x33, 0xAAAA_0003),
        s([0x0005_10D1, 0, 0], 0x44, 0xAAAA_0004),
    ])
}

#[test]
fn the_section_for_this_sku_and_hardware_is_chosen() {
    let f = so_file();
    let p = parse(&f, [0x0006_10D1, 3, 0], MAC_SO, RF_GF).expect("a section");
    assert_eq!(p.version, 0xAAAA_0002);
    assert_eq!(p.chunks.len(), 2);
    assert!(p.chunks[0].iter().all(|&b| b == 0x22) && p.chunks[0].len() == 6830);
    assert!(p.chunks[1].iter().all(|&b| b == 0xDD) && p.chunks[1].len() == 6826);
    // SO-F matches the same section.
    assert_eq!(parse(&f, [0x0005_10D1, 0, 0], MAC_SOF, RF_GF).unwrap().version, 0xAAAA_0004);
}

#[test]
fn no_section_for_another_sku_or_hardware() {
    let f = so_file();
    assert!(parse(&f, [0x0007_10D1, 0, 0], MAC_SO, RF_GF).is_none(), "unknown SKU");
    assert!(parse(&f, [0x0006_10D1, 0, 0], 0x42, RF_GF).is_none(), "a TY MAC in the SO file");
    assert!(parse(&f, [0x0006_10D1, 0, 0], MAC_SO, 0x10A).is_none(), "an HR radio");
    assert!(parse(&[], [0x0006_10D1, 0, 0], MAC_SO, RF_GF).is_none());
}

#[test]
fn a_later_section_for_the_same_sku_is_used_when_the_first_is_for_other_hardware() {
    let f = file(&[
        Section { sku: [9, 0, 0], hw: vec![(0x42, RF_GF)], chunks: vec![vec![1; 8]], version: 1 },
        Section { sku: [9, 0, 0], hw: vec![(MAC_SO, RF_GF)], chunks: vec![vec![2; 8]], version: 2 },
    ]);
    assert_eq!(parse(&f, [9, 0, 0], MAC_SO, RF_GF).unwrap().version, 2);
}

#[test]
fn the_deprecated_separator_is_not_a_chunk() {
    let mut f = Vec::new();
    tlv(&mut f, 64, &[7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    tlv(&mut f, 58, &[0x37, 0, 0x0D, 0x01]);
    tlv(&mut f, 19, &0xDDDD_EEEEu32.to_le_bytes());
    tlv(&mut f, 19, &[0, 0, 0, 0, 0xAB, 0xCD]);
    let p = parse(&f, [7, 0, 0], MAC_SO, RF_GF).unwrap();
    assert_eq!(p.chunks, vec![&[0xAB, 0xCD][..]]);
}

#[test]
fn a_section_without_chunks_or_with_too_many_is_refused() {
    let none =
        file(&[Section { sku: [1, 0, 0], hw: vec![(MAC_SO, RF_GF)], chunks: vec![], version: 0 }]);
    assert!(parse(&none, [1, 0, 0], MAC_SO, RF_GF).is_none());
    let many = file(&[Section {
        sku: [1, 0, 0],
        hw: vec![(MAC_SO, RF_GF)],
        chunks: vec![vec![0; 4]; MAX_CHUNKS + 1],
        version: 0,
    }]);
    assert!(parse(&many, [1, 0, 0], MAC_SO, RF_GF).is_none());
}

#[test]
fn a_truncated_file_is_read_no_further_than_it_goes() {
    let f = so_file();
    for cut in [1, 7, 8, 20, f.len() / 2, f.len() - 1] {
        // Never panics; a cut inside the wanted section loses it.
        let _ = parse(&f[..cut], [0x0005_10D1, 0, 0], MAC_SO, RF_GF);
    }
    // The last chunk's value ends two bytes before the file does (padding):
    // losing only padding loses nothing, as in Linux, which checks the value's
    // length; cutting into the value loses the section that holds it.
    assert!(parse(&f[..f.len() - 2], [0x0005_10D1, 0, 0], MAC_SO, RF_GF).is_some());
    assert!(parse(&f[..f.len() - 3], [0x0005_10D1, 0, 0], MAC_SO, RF_GF).is_none());
    // An earlier, intact section is found before the broken TLV is reached,
    // and used, as Linux's parse returns at its first good match.
    assert!(parse(&f[..f.len() - 3], [0x0006_10D1, 0, 0], MAC_SO, RF_GF).is_some());
}

#[test]
fn the_reserve_covers_every_sku_for_this_hardware() {
    let f = so_file();
    let r = reserve(&f, MAC_SO, RF_GF);
    assert_eq!(r, PAGE + 2 * 2 * PAGE, "the descriptor page and two 2-page chunks");
    for sku in [[0x0006_10D1, 0, 0], [0x0006_10D1, 3, 0], [0x000A_10D1, 0, 0], [0x0005_10D1, 0, 0]]
    {
        assert!(area_len(&parse(&f, sku, MAC_SO, RF_GF).unwrap()) <= r);
    }
    assert_eq!(reserve(&f, 0x42, RF_GF), 0, "nothing for another MAC");
}

#[test]
fn the_payloads_land_page_aligned_with_their_addresses_in_the_descriptor() {
    let f = so_file();
    let p = parse(&f, [0x0006_10D1, 0, 0], MAC_SO, RF_GF).unwrap();
    let m = Mem::new(8 * PAGE, 0x1000_0000);
    let placed = lay_out(m.as_ref(), PAGE, &p).expect("fits");
    assert_eq!(placed, Placed { desc: 0x1000_0000 + PAGE as u64, size: 6830 + 6826 });
    let mut desc = [0u8; DESC_LEN];
    m.read(PAGE, &mut desc);
    let a0 = u64::from_le_bytes(desc[0..8].try_into().unwrap());
    let a1 = u64::from_le_bytes(desc[8..16].try_into().unwrap());
    assert_eq!(a0, 0x1000_0000 + 2 * PAGE as u64);
    assert_eq!(a1, 0x1000_0000 + 4 * PAGE as u64);
    assert!(desc[16..].iter().all(|&b| b == 0), "the rest of the array is zero");
    let mut c0 = vec![0u8; 6830];
    m.read(2 * PAGE, &mut c0);
    assert_eq!(&c0[..], p.chunks[0]);
    let mut c1 = vec![0u8; 6826];
    m.read(4 * PAGE, &mut c1);
    assert_eq!(&c1[..], p.chunks[1]);
}

#[test]
fn a_layout_that_does_not_fit_or_is_misaligned_is_refused() {
    let f = so_file();
    let p = parse(&f, [0x0006_10D1, 0, 0], MAC_SO, RF_GF).unwrap();
    assert!(lay_out(Mem::new(4 * PAGE, 0).as_ref(), 0, &p).is_none(), "five pages needed");
    assert!(lay_out(Mem::new(8 * PAGE, 0).as_ref(), 100, &p).is_none(), "not page aligned");
}

#[test]
fn the_scratch_takes_the_descriptor_address_and_total_size() {
    let m = Mem::new(2 * PAGE, 0);
    assert!(point_scratch(m.as_ref(), PAGE, Placed { desc: 0x1234_5000, size: 13656 }));
    let mut b = [0u8; 12];
    m.read(PAGE + 16, &mut b);
    assert_eq!(u64::from_le_bytes(b[0..8].try_into().unwrap()), 0x1234_5000);
    assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 13656);
}

/* The boot, against the modeled card: ALIVE names SKU 0x000610D1, so the
section is laid out after the image loader and the scratch points at it before
the doorbell rings. */
#[test]
fn a_sku_part_is_given_its_section_before_the_doorbell() {
    let f: &'static [u8] = Box::leak(so_file().into_boxed_slice());
    let ucode = Ucode::parse(SO_GF).unwrap();
    let res = reserve(f, MAC_SO, RF_GF);
    let ctrl = Mem::new(control_len_pnvm(ucode.iml.len(), res).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let layout = classify(SO_GF);
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw: Vec<Rc<Mem>> = sizes
        .iter()
        .enumerate()
        .map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000))
        .collect();
    let model = Model::new(ctrl, rbs);
    model.s.borrow_mut().alive_sku = [0x0006_10D1, 0, 0];
    let fw_refs: Vec<&Mem> = fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: model.ctrl.as_ref(), rbs: model.rbs.as_ref(), fw: &fw_refs };
    let mut dev = Dev::new(&model, model.ctrl.as_ref(), model.rbs.as_ref());
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x0010_D000, pnvm: Some(f) };
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board).expect("alive and the PNVM step done");
    assert_eq!(dev.pnvm, Some(0xAAAA_0001));
    let off = pnvm_off(ucode.iml.len()).unwrap();
    let mut b = [0u8; 12];
    model.ctrl.read(PRPH_SCRATCH + 16, &mut b);
    assert_eq!(u64::from_le_bytes(b[0..8].try_into().unwrap()), 0x1000_0000 + off as u64);
    assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 6830 + 6826);
    let mut first = [0u8; 8];
    model.ctrl.read(off, &mut first);
    let chunk0 = u64::from_le_bytes(first) - 0x1000_0000;
    let mut c0 = vec![0u8; 6830];
    model.ctrl.read(chunk0 as usize, &mut c0);
    assert!(c0.iter().all(|&x| x == 0x11));
    assert!(model.s.borrow().prph.contains(&(0xD0_5C04, 1 << 20)), "the doorbell rang");
}

/* With no section for the SKU the doorbell still rings, and the scratch keeps
its zero pnvm fields: the firmware runs on its defaults, as Linux's does. */
#[test]
fn an_unknown_sku_boots_on_firmware_defaults() {
    let f: &'static [u8] = Box::leak(so_file().into_boxed_slice());
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(
        control_len_pnvm(ucode.iml.len(), reserve(f, MAC_SO, RF_GF)).unwrap(),
        0x1000_0000,
    );
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let layout = classify(SO_GF);
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw: Vec<Rc<Mem>> = sizes
        .iter()
        .enumerate()
        .map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000))
        .collect();
    let model = Model::new(ctrl, rbs);
    model.s.borrow_mut().alive_sku = [0x0009_10D1, 0, 0];
    let fw_refs: Vec<&Mem> = fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: model.ctrl.as_ref(), rbs: model.rbs.as_ref(), fw: &fw_refs };
    let mut dev = Dev::new(&model, model.ctrl.as_ref(), model.rbs.as_ref());
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x0010_D000, pnvm: Some(f) };
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board).expect("alive");
    assert_eq!(dev.pnvm, None);
    let mut b = [0u8; 12];
    model.ctrl.read(PRPH_SCRATCH + 16, &mut b);
    assert!(b.iter().all(|&x| x == 0));
    assert!(model.s.borrow().prph.contains(&(0xD0_5C04, 1 << 20)));
}
