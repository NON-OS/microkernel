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

//! The quote's signed structure: one that parses is a TPM-generated quote
//! whose fields all lie inside the input, within their caps.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_attest::quote::Attest;

fuzz_target!(|data: &[u8]| {
    if let Ok(a) = Attest::parse(data) {
        assert_eq!(&data[..6], &[0xFF, 0x54, 0x43, 0x47, 0x80, 0x18]);
        assert!(a.signer.len() <= 66 && a.extra.len() <= 66 && a.pcr_digest.len() <= 66);
        assert!(a.selection.count >= 1 && a.selection.count <= 16);
        assert!(a.selection.bitmap.len() <= 4);
        let tail = 2 + a.pcr_digest.len();
        assert_eq!(&data[data.len() - a.pcr_digest.len()..], a.pcr_digest);
        assert!(data.len() >= 6 + tail);
    }
});
