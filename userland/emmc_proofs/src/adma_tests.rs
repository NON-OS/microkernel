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

//! ADMA2 descriptor tables: both widths, lengths, End on the last only, no
//! Nop, and splitting. Sizes, alignment and reach are `bounds`.

use crate::emmc::sdhci::adma::*;
#[path = "adma_bounds_tests.rs"]
mod bounds;

fn descs(t: &[u8], w: Width, n: usize) -> Vec<(u16, u16, u64)> {
    let dl = w.desc_len();
    (0..n)
        .map(|i| {
            let d = &t[i * dl..(i + 1) * dl];
            let addr = match w {
                Width::A32 => u32::from_le_bytes(d[4..8].try_into().unwrap()) as u64,
                Width::A64 => u64::from_le_bytes(d[4..12].try_into().unwrap()),
            };
            (u16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]), addr)
        })
        .collect()
}

/* The descriptor bits the driver never sets, from the SD Host Controller
spec (1.13.4): interrupt on completion, and the action field's mask. */
const ATTR_INT: u16 = 1 << 2;
const ACT_MASK: u16 = 3 << 4;

#[test]
fn one_sector_is_one_transfer_descriptor_with_end() {
    for w in [Width::A32, Width::A64] {
        let mut t = [0u8; TABLE_BYTES];
        assert_eq!(build(&mut t, w, 0x20_0000, 512), Ok(1));
        assert_eq!(descs(&t, w, 1), vec![(0x23, 512, 0x20_0000)]);
    }
}

#[test]
fn a_buffer_is_split_into_32_kib_pieces_end_on_the_last_only() {
    for w in [Width::A32, Width::A64] {
        for len in [32768usize, 32768 + 512, 65536, 100 * 1024, 512 * 1024] {
            let mut t = [0u8; TABLE_BYTES];
            let n = build(&mut t, w, 0x40_0000, len).unwrap();
            assert_eq!(n, len.div_ceil(MAX_CHUNK));
            let d = descs(&t, w, n);
            let mut total = 0usize;
            for (i, &(attr, l, addr)) in d.iter().enumerate() {
                assert_eq!(attr & ACT_MASK, ACT_TRAN, "never Nop or Link");
                assert_eq!(attr & ATTR_VALID, ATTR_VALID);
                assert_eq!(attr & ATTR_INT, 0);
                assert_eq!(attr & ATTR_END != 0, i + 1 == n, "End only on the last");
                assert!(l != 0 && l as usize <= MAX_CHUNK);
                assert_eq!(addr, 0x40_0000 + total as u64);
                total += l as usize;
            }
            assert_eq!(total, len);
            // Nothing past the table is written.
            assert!(t[n * w.desc_len()..].iter().all(|&b| b == 0));
        }
    }
}
