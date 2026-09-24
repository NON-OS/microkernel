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

//! Decoding one labelled base64 block out of a region.

extern crate alloc;

use alloc::vec::Vec;
use base64ct::{Base64, Encoding};

use super::find::find;

pub(super) fn pem(region: &[u8], label: &[u8]) -> Vec<u8> {
    let open = [b"-----BEGIN ".as_slice(), label, b"-----"].concat();
    let close = [b"-----END ".as_slice(), label, b"-----"].concat();
    let from = find(region, &open, 0).expect("block opens") + open.len();
    let to = find(region, &close, from).expect("block closes");
    let packed: Vec<u8> =
        region[from..to].iter().copied().filter(|b| !b.is_ascii_whitespace()).collect();
    Base64::decode_vec(core::str::from_utf8(&packed).expect("base64 is ascii")).expect("base64")
}
