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

//! The four store artifacts of a name, served by the vfs stand-in.

use crate::clients::vfs;

/// The cap artifacts.rs reads each artifact under (its MAX_ARTIFACT).
pub const MAX_ARTIFACT: u32 = 16 * 1024 * 1024;
pub const EXTS: [&[u8]; 4] = [b".elf", b".nonos_id_cert.bin", b".manifest.bin", b".zk_trailer.bin"];

pub const NAME: &[u8] = b"demo_app";

pub fn path(name: &[u8], ext: &[u8]) -> Vec<u8> {
    [b"/capsules/", name, ext].concat()
}

/// Artifact i of name holds 4 * (i + 1) bytes of i + 1; skip leaves one out.
pub fn put_artifacts(name: &[u8], skip: Option<usize>) {
    vfs::reset();
    for (i, ext) in EXTS.iter().enumerate().filter(|(i, _)| Some(*i) != skip) {
        vfs::put(&path(name, ext), artifact(i));
    }
}

pub fn artifact(i: usize) -> Vec<u8> {
    vec![i as u8 + 1; 4 * (i + 1)]
}
