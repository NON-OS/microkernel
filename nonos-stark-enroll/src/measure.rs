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

//! A loader's measurement: the PE Authenticode SHA-256 the firmware extends
//! PCR 4 with when it starts the loader, so the kernel can rebuild it from the
//! TCG log.

use nonos_boot_measure::authenticode::digest;

use crate::io::{die, read};

/*
 * A signing tool pads an image to eight bytes before it appends the
 * certificate table, and the firmware hashes the image as it finds it. An image
 * whose length is not a multiple of eight and that carries bytes past its
 * sections therefore has one digest unsigned and another signed, and would be
 * enrolled under a measurement the signed loader never produces. Such an image
 * is refused, so the enrolled digest holds whether the loader is signed or not.
 */
pub fn authenticode_of(path: &str) -> [u8; 32] {
    let f = read(path);
    let d = digest(&f).unwrap_or_else(|e| {
        die(&format!("{path} has no Authenticode digest: {e:?} ({})", e.code()))
    });
    let mut padded = f.clone();
    padded.resize(f.len().next_multiple_of(8), 0);
    if digest(&padded) != Ok(d) {
        die(&format!(
            "{path}: its Authenticode digest changes when it is signed; pad it to 8 bytes first"
        ));
    }
    d
}
