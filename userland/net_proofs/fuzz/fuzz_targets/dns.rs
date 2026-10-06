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

//! Any bytes as a DNS response: the reader never panics and never loops on
//! compression pointers (a hang is a libFuzzer timeout), a skipped name ends
//! inside the message, and an answer never carries the other family's address.

#![no_main]

use libfuzzer_sys::fuzz_target;
use net_proofs::dns::{first_address, question_matches, skip, HDR_LEN, TYPE_A, TYPE_AAAA};

fuzz_target!(|data: &[u8]| {
    if let Ok((_, Some(a))) = first_address(data) {
        assert!(a.ipv4.is_none() || a.rtype == TYPE_A);
        assert!(a.ipv6.is_none() || a.rtype == TYPE_AAAA);
    }
    for start in [HDR_LEN, data.len() / 2] {
        if let Ok(end) = skip(data, start) {
            assert!(end > start && end <= data.len(), "a name skipped to {end}");
        }
    }
    let (q, r) = data.split_at(data.len() / 2);
    let _ = question_matches(q, r);
});
