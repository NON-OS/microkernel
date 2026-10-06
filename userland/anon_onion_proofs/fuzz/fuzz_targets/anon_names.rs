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


//! The signed names list and the HTTP answer it arrives in. A list that
//! verifies under keys the fuzzer cannot hold would be a forgery, so the
//! target asserts none does, while every reader runs over the bytes.

#![no_main]

use anon_onion_proofs::onion::fetch::body_within;
use anon_onion_proofs::onion::names::document::is_short_name;
use anon_onion_proofs::onion::names::{defaults, normal, verify, LIST_MAX};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let signers = defaults::signers();
    assert!(verify(data, &signers, 1_791_028_800).is_err(), "a list verified under the six services' keys");
    if let Ok(body) = body_within(data, LIST_MAX) {
        assert!(verify(body, &signers, 1_791_028_800).is_err());
    }
    let _ = is_short_name(&normal(data));
});
