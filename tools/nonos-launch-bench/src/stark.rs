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

//! The capsule attestation check, `capsule_attest::path::verify_against`,
//! on the committed nonos-proof-verify fixture: a real STARK proof over a
//! capsule slot, accepted against its development root.

use std::fs;
use std::path::Path;

use crate::security::capsule_attest::verify_against;
use crate::stats::{row, time};

pub fn run(root: &Path, capsules: &[crate::inputs::Capsule]) {
    let sizes: Vec<usize> = capsules.iter().map(|c| c.trailer.len()).collect();
    if let (Some(lo), Some(hi)) = (sizes.iter().min(), sizes.iter().max()) {
        println!("# committed capsule trailers: {} B to {} B", lo, hi);
    }
    let fx = root.join("nonos-proof-verify/tests/fixture");
    let read = |n: &str| fs::read(fx.join(n));
    let (Ok(elf), Ok(trailer), Ok(root_bytes)) = (read("a.elf"), read("a.trailer"), read("caps_root.bin")) else {
        println!("stark fixture missing under {}", fx.display());
        return;
    };
    let Ok(policy) = <[u8; 32]>::try_from(root_bytes.as_slice()) else {
        println!("caps_root.bin is not 32 bytes");
        return;
    };
    let digest = *blake3::hash(&elf).as_bytes();
    match verify_against(&trailer, &digest, 0x18, &policy) {
        Ok(_) => {}
        Err(e) => {
            println!("stark fixture refused: {}", e.as_str());
            return;
        }
    }
    let s = time(|| {
        crate::security::admission_cache::forget_all();
        let _ = std::hint::black_box(verify_against(&trailer, &digest, 0x18, &policy));
    });
    row(&format!("attestation verify (STARK, {} B trailer)", trailer.len()), 1, &s);
    let w = time(|| {
        let _ = std::hint::black_box(verify_against(&trailer, &digest, 0x18, &policy));
    });
    row("attestation verify, acceptance remembered", 1, &w);
    let mut other = digest;
    other[0] ^= 1;
    if verify_against(&trailer, &other, 0x18, &policy).is_ok() {
        println!("attestation: a changed digest was accepted from the cache");
    }
}
