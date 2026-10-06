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

//! Bytes off the wire: record framing never panics and every record it
//! frames ends inside the input; the server hello's key share is read
//! without a panic whatever the handshake bytes are.

#![no_main]

use libfuzzer_sys::fuzz_target;
use tls_proofs::{record_frame, server_hello};

fuzz_target!(|data: &[u8]| {
    let mut at = 0;
    while let Some((_, end)) = record_frame::record_at(data, at) {
        assert!(at + 5 <= end && end <= data.len());
        at = end;
    }
    let _ = server_hello::key_share(data);
});
