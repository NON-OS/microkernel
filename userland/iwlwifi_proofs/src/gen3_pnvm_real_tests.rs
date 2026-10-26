// NONOS Operating System (AGPL-3.0-or-later)
//! The AX211's PNVM, linux-firmware's iwlwifi-so-a0-gf-a0.pnvm, as the driver
//! carries it: for every SKU the file holds, an SO or SO-F MAC with a GF radio
//! gets exactly that section's chunks and version, read here by a walk of the
//! file's TLVs written apart from the driver's; the reserve covers each; an HR
//! radio or another MAC gets none.

use crate::firmware::blob::gen3_pnvm;
use crate::gen3::pnvm::{area_len, parse, reserve};
use crate::gen3::select::Image;

const MAC_SO: u16 = 0x37;
const MAC_SOF: u16 = 0x43;
const RF_GF: u16 = 0x10D;

struct Section {
    sku: [u32; 3],
    version: u32,
    chunks: Vec<Vec<u8>>,
}

// The file's sections: a SKU TLV (64) opens one, then its version (62) and
// its data chunks (19, a four-byte offset and the bytes).
fn sections(f: &[u8]) -> Vec<Section> {
    let (mut out, mut at): (Vec<Section>, usize) = (Vec::new(), 0);
    while at + 8 <= f.len() {
        let w = |o: usize| u32::from_le_bytes(f[o..o + 4].try_into().unwrap());
        let (ty, len) = (w(at), w(at + 4) as usize);
        let v = &f[at + 8..at + 8 + len];
        match ty {
            64 => out.push(Section {
                sku: [w(at + 8), w(at + 12), w(at + 16)],
                version: 0,
                chunks: vec![],
            }),
            62 => out.last_mut().unwrap().version = u32::from_le_bytes(v.try_into().unwrap()),
            19 => out.last_mut().unwrap().chunks.push(v[4..].to_vec()),
            _ => {}
        }
        at += 8 + len.div_ceil(4) * 4;
    }
    out
}

#[test]
fn every_sku_in_the_ax211_file_gets_its_own_section() {
    let f = gen3_pnvm(Image::SoGf).expect("the AX211 PNVM is carried");
    let all = sections(f);
    assert!(all.len() >= 4, "{} sections", all.len());
    let r = reserve(f, MAC_SO, RF_GF);
    for s in &all {
        for mac in [MAC_SO, MAC_SOF] {
            let p = parse(f, s.sku, mac, RF_GF).expect("a section for this SKU");
            assert_eq!(p.version, s.version, "{:x?}", s.sku);
            let got: Vec<Vec<u8>> = p.chunks.iter().map(|c| c.to_vec()).collect();
            assert_eq!(got, s.chunks, "{:x?}", s.sku);
            assert!(area_len(&p) <= r, "the reserve covers {:x?}", s.sku);
        }
    }
}

#[test]
fn no_section_for_another_radio_or_mac_and_no_file_for_hr() {
    let f = gen3_pnvm(Image::SoGf).expect("the AX211 PNVM is carried");
    let sku = sections(f)[0].sku;
    assert!(parse(f, sku, MAC_SO, 0x10A).is_none(), "an HR radio");
    assert!(parse(f, sku, 0x42, RF_GF).is_none(), "another MAC");
    assert!(parse(f, [0xDEAD_BEEF, 0, 0], MAC_SO, RF_GF).is_none(), "another SKU");
    assert!(gen3_pnvm(Image::SoHr).is_none(), "an HR radio has no PNVM");
}
