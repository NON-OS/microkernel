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

//! Members for the tests: tars of a whole number of blocks, one stored and
//! one "compressed" (the parser does not decompress, so any bytes do).


use nonos_app_image::{build, chunk_list, Codec, Part};

pub fn tar(seed: u8, blocks: usize) -> Vec<u8> {
    (0..blocks * 512).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed)).collect()
}

pub struct Fixture {
    pub tars: Vec<Vec<u8>>,
    pub xz: Vec<u8>,
}

pub fn fixture() -> Fixture {
    Fixture { tars: vec![tar(1, 3), tar(2, 1), tar(3, 200)], xz: b"\xfd7zXZ\0 stands in".to_vec() }
}

pub fn parts(f: &Fixture) -> Vec<Part<'_>> {
    vec![
        Part { name: b"telegram-desktop", codec: Codec::Tar, tar: &f.tars[0], stored: &f.tars[0] },
        Part { name: b"qt6-qtbase", codec: Codec::Xz, tar: &f.tars[1], stored: &f.xz },
        Part { name: b"libstdc++", codec: Codec::Tar, tar: &f.tars[2], stored: &f.tars[2] },
    ]
}

pub fn image() -> Vec<u8> {
    build(&parts(&fixture())).expect("built")
}

pub fn list(image: &[u8], chunk: u32) -> (Vec<u8>, [u8; 32]) {
    let l = chunk_list(image, chunk).expect("listed");
    let pin = *blake3::hash(&l).as_bytes();
    (l, pin)
}
