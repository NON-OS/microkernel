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

//! Any bytes as an EDID, through the base block, CTA-861 and DisplayID
//! readers and the loader's own base block reader. None panics or reads
//! past the bytes. The first input byte picks whether the base header is
//! written and every block's checksum sealed, so the mutations reach the
//! readers behind those gates as well as the gates themselves. What parse
//! keeps is a whole number of blocks and a prefix of the input; every
//! extension it hands out lies inside that and has a good checksum; a text
//! descriptor is at most 13 bytes; and the loader picks the same native
//! mode and size as the kernel's reader for every base block.

#![no_main]

use edid_proofs::blocks::{checksum_ok, BLOCK, HEADER};
use edid_proofs::edid::Edid;
use edid_proofs::loader_pick::parse_edid;
use libfuzzer_sys::fuzz_target;

fn shape(data: &[u8]) -> Vec<u8> {
    let Some((&mode, rest)) = data.split_first() else { return Vec::new() };
    let mut raw = rest.to_vec();
    if mode & 1 != 0 && raw.len() >= HEADER.len() {
        raw[..HEADER.len()].copy_from_slice(&HEADER);
    }
    if mode & 2 != 0 {
        for b in raw.chunks_exact_mut(BLOCK) {
            b[BLOCK - 1] = 0;
            let sum = b.iter().fold(0u8, |s, &v| s.wrapping_add(v));
            b[BLOCK - 1] = sum.wrapping_neg();
        }
    }
    raw
}

fn kernel(e: &Edid, raw: &[u8]) {
    let kept = e.raw();
    assert!(kept.len() <= raw.len() && kept.len() % BLOCK == 0 && raw.starts_with(kept));
    assert!(e.extension_count() < kept.len() / BLOCK);
    let (lo, hi) = (kept.as_ptr() as usize, kept.as_ptr() as usize + kept.len());
    for ext in e.extensions() {
        let at = ext.as_ptr() as usize;
        assert!(at >= lo + BLOCK && at + BLOCK <= hi, "an extension outside the kept bytes");
        assert!(checksum_ok(ext));
    }
    let mut n = 0usize;
    e.for_each_timing(&mut |_| n += 1);
    e.for_each_audio(&mut |_| n += 1);
    for text in [e.name(), e.serial_text()].into_iter().flatten() {
        assert!(text.as_bytes().len() <= 13);
    }
    let _ = (e.preferred(), e.range_limits(), e.tile(), e.max_tmds_khz());
    let _ = (e.physical_mm(), e.basic_audio(), e.size_mm(), n);
}

fuzz_target!(|data: &[u8]| {
    let raw = shape(data);
    let ours = Edid::parse(&raw).ok();
    if let Some(e) = &ours {
        kernel(e, &raw);
    }
    let ours = ours.and_then(|e| e.native()).map(|n| (n.width, n.height, n.width_mm, n.height_mm));
    let theirs = parse_edid(&raw).map(|e| (e.width, e.height, e.width_mm, e.height_mm));
    assert_eq!(ours, theirs, "the loader and the kernel read a different native mode");
});
