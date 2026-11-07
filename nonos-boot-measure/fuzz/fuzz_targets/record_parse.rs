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

//! Any bytes as an `.approval` record, straight to the parser the loader and
//! the kernel run before any signature check: it never panics, it accepts
//! exactly the records of 104 bytes whose four root words are field elements,
//! and what it accepts reads back to the same bytes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_boot_measure::record::{parse, RECORD_LEN};

const P: u64 = 0xFFFF_FFFF_0000_0001;

fuzz_target!(|data: &[u8]| {
    let canonical = data.len() == RECORD_LEN
        && data[..32].chunks_exact(8).all(|w| u64::from_le_bytes(w.try_into().unwrap_or([0xff; 8])) < P);
    match parse(data) {
        Ok(rec) => {
            assert!(canonical, "accepted a record it should refuse");
            let mut back = Vec::with_capacity(RECORD_LEN);
            back.extend_from_slice(&rec.root);
            back.extend_from_slice(&rec.epoch.to_le_bytes());
            back.extend_from_slice(&rec.r);
            back.extend_from_slice(&rec.s);
            assert_eq!(back, data, "read back to other bytes");
        }
        Err(_) => assert!(!canonical, "refused a canonical record"),
    }
});
