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

//! The hellos: a hello that parses is exactly its length, and encoding it
//! again gives back the very bytes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_wire::{ClientHello, ServerHello, CLIENT_HELLO_LEN, SERVER_HELLO_LEN};

fuzz_target!(|data: &[u8]| {
    let mut out = [0u8; CLIENT_HELLO_LEN];
    if let Ok(h) = ClientHello::parse(data) {
        assert_eq!(h.encode(&mut out), Ok(CLIENT_HELLO_LEN));
        assert_eq!(&out[..], data);
    }
    if let Ok(h) = ServerHello::parse(data) {
        assert_eq!(h.encode(&mut out), Ok(SERVER_HELLO_LEN));
        assert_eq!(&out[..SERVER_HELLO_LEN], data);
    }
});
