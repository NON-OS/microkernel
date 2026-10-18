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

//! The parsers of what the card and the reader send, swept with hostile
//! input: response bytes of every length and type, and every 128-bit CSD
//! and CID. None panics, and what each returns stays in its bounds.

use super::rng::Rng;
use crate::regs::sd::SD_RSP_TYPE_R2;
use crate::sd::{addressable, capacity_blocks, Cid};
use crate::wire::{parse, ResponseError};

#[test]
fn the_response_parser_takes_any_bytes_of_any_length_and_type() {
    let mut rng = Rng::new(0x5D);
    for _ in 0..200_000 {
        let results = rng.bytes(25);
        let rsp = rng.next() as u8;
        match parse(&results, rsp) {
            Ok(r) if rsp == SD_RSP_TYPE_R2 => assert_eq!(r.words[3] & 0xFF, 1),
            Ok(_) => {}
            Err(ResponseError::Short) => assert!(results.len() < 18),
            Err(_) => {}
        }
    }
}

#[test]
fn any_csd_gives_a_size_that_cannot_wrap_and_is_addressed_only_when_it_fits() {
    let mut rng = Rng::new(0xC5D);
    for _ in 0..200_000 {
        let csd = rng.words();
        let Some(blocks) = capacity_blocks(&csd) else { continue };
        assert!(blocks <= 1 << 38, "{csd:x?}");
        for hc in [false, true] {
            let limit = if hc { 1u64 << 32 } else { 1 << 23 };
            assert_eq!(addressable(blocks, hc), blocks > 0 && blocks <= limit);
        }
    }
}

#[test]
fn any_cid_names_a_printable_product_and_eight_hex_digits() {
    let mut rng = Rng::new(0xC1D);
    for _ in 0..200_000 {
        let words = rng.words();
        let cid = Cid::new(words);
        assert!(cid.product().iter().all(|&b| b == b' ' || b.is_ascii_graphic()));
        let serial = cid.serial();
        assert!(serial.iter().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_lowercase()));
        let psn = u32::from_str_radix(core::str::from_utf8(&serial).unwrap_or(""), 16);
        assert_eq!(psn, Ok(words[2] << 8 | words[3] >> 24));
        assert_eq!(format!("{cid:?}"), "Cid(..)");
    }
}
