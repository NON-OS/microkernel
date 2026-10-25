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

//! A version 5 document: one that parses is no larger than the cap, is
//! exactly as long as its fields say, carries the PCR list, and
//! its every field lies inside the input.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_attest::consts::{MAX_DOC, PCR_COUNT};
use nsh_attest::{Doc, Refusal};

/// Magic, version, challenge, root, count, three flags, grants, identity.
const HEAD: usize = 8 + 4 + 32 + 32 + 4 + 3 + 4 + 4;

fuzz_target!(|data: &[u8]| {
    match Doc::parse(data) {
        Ok(d) => {
            assert!(data.len() <= MAX_DOC);
            let fixed = HEAD + 1 + 33 * PCR_COUNT + 4 * 4 + 64;
            let n = fixed + d.log.len() + d.attest.len() + d.signature.len();
            assert_eq!(n, data.len(), "every byte is a field's");
            assert_eq!(&data[..12], b"NONOSATT\0\0\0\x05");
            assert!(!d.attest.is_empty() && !d.signature.is_empty());
            assert!(d.pcr(9).is_some() && d.pcr(3).is_none());
        }
        Err(Refusal::Version(v)) => assert_ne!(v, 5),
        Err(Refusal::Format(_)) => {}
        Err(other) => panic!("a parse refused with {other:?}"),
    }
});
