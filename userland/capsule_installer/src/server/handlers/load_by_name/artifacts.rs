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

//! The four store artifacts a load by name reads, and the load request over them.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::CapsuleLoadRequest;

// A capsule ELF can be several MB. The artifacts are read here, one file at a
// time, so a multi-MB ELF never has to cross the IPC boundary in a single
// message: the caller sends only the name, and the verified blobs are read
// straight from the store.
const MAX_ARTIFACT: u32 = 16 * 1024 * 1024;

pub(super) struct Artifacts {
    elf: Vec<u8>,
    cert: Vec<u8>,
    manifest: Vec<u8>,
    trailer: Vec<u8>,
}

impl Artifacts {
    /// /capsules/<name>.{elf,nonos_id_cert.bin,manifest.bin,zk_trailer.bin}, all four read.
    pub(super) fn read(pid: u32, name: &[u8]) -> Option<Self> {
        let (Ok(elf), Ok(cert), Ok(manifest), Ok(trailer)) = (
            read_artifact(pid, name, b".elf"),
            read_artifact(pid, name, b".nonos_id_cert.bin"),
            read_artifact(pid, name, b".manifest.bin"),
            read_artifact(pid, name, b".zk_trailer.bin"),
        ) else {
            return None;
        };
        Some(Self { elf, cert, manifest, trailer })
    }

    pub(super) fn request(&self, caps: u64, args: &[u8], on_behalf_of: u32) -> CapsuleLoadRequest {
        CapsuleLoadRequest {
            elf_ptr: self.elf.as_ptr() as u64,
            cert_ptr: self.cert.as_ptr() as u64,
            manifest_ptr: self.manifest.as_ptr() as u64,
            trailer_ptr: self.trailer.as_ptr() as u64,
            requested_caps: caps,
            args_ptr: if args.is_empty() { 0 } else { args.as_ptr() as u64 },
            elf_len: self.elf.len() as u32,
            cert_len: self.cert.len() as u32,
            manifest_len: self.manifest.len() as u32,
            trailer_len: self.trailer.len() as u32,
            args_len: args.len() as u32,
            on_behalf_of,
        }
    }
}

fn read_artifact(pid: u32, name: &[u8], ext: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut path = Vec::with_capacity(10 + name.len() + ext.len());
    path.extend_from_slice(b"/capsules/");
    path.extend_from_slice(name);
    path.extend_from_slice(ext);
    read_file(pid, &path, MAX_ARTIFACT)
}
