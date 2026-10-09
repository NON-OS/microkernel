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

use super::pe_build::{fields, image, put, LFANEW, OPT};
use crate::authenticode::{digest, PeError};

#[test]
fn every_truncation_and_every_malformed_header_is_refused_without_a_panic() {
    let f = image(true, &[(0x200, 0x100, 0xAA)]);
    let honest = digest(&f).expect("digests");
    (0..f.len()).for_each(|n| assert_ne!(digest(&f[..n]).ok(), Some(honest), "prefix {n}"));
    let bad = |at: usize, v: &[u8], e: PeError| {
        let mut m = f.clone();
        put(&mut m, at, v);
        assert_eq!(digest(&m), Err(e), "at {at:#x}");
    };
    bad(0, b"ZM", PeError::NotPe);
    bad(LFANEW, b"PX", PeError::NotPe);
    bad(0x3C, &0xFFFF_FFF0u32.to_le_bytes(), PeError::NotPe);
    bad(OPT, &0x10Cu16.to_le_bytes(), PeError::BadOptionalHeader);
    bad(LFANEW + 6, &97u16.to_le_bytes(), PeError::TooManySections);
    bad(OPT + 60, &0x10_0000u32.to_le_bytes(), PeError::OutOfFile);
    bad(fields(true).1 + 4, &0x10_0000u32.to_le_bytes(), PeError::OutOfFile);
}
