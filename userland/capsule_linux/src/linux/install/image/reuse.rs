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

//! An image installed again: when this machine's record for the pin says
//! which packs it placed, and every one of them is still in the store with
//! a trailer this machine minted over its head and the head the record
//! names, the install registers the closure and is done. Nothing is
//! fetched, decoded or placed. Any doubt sends the install the long way.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::{mk_getpid, mk_local_verify};
use nonos_linux_pack::{Index, HEADER_LEN};

use super::super::record::vouch::mint;
use super::named::hex;
use super::placed::{decode, encode, Placed, MAX_BYTES};
use crate::linux::file::family::chosen;
use crate::linux::file::pack::{pack_path, trailer_of};

const DIR: &[u8] = b"/nonos/linux/image-cache";
const MAX_TRAILER: u32 = 4096;

fn at(pin: &[u8; 32]) -> Vec<u8> {
    [DIR, b"/", hex(pin).as_bytes(), b"/placed"].concat()
}

/// A pack's header and index as the store holds them, if this machine's
/// trailer beside it holds them: the check a run makes before it serves it.
fn head(name: &str) -> Option<Vec<u8>> {
    let (pid, path) = (mk_getpid(), pack_path(chosen(), name));
    let (len, _) = vfs::stat(pid, &path).ok()?;
    let len = usize::try_from(len).ok()?;
    let mut stream = VfsStream::open(pid, &path).ok()?;
    let first = stream.read_window(0, HEADER_LEN as u32).ok()?;
    let head_len = Index::data_start(&first, len).ok()?;
    let head = stream.read_window(0, u32::try_from(head_len).ok()?).ok()?;
    let trailer = vfs::read_file(pid, &trailer_of(&path), MAX_TRAILER).ok()?;
    (head.len() == head_len && mk_local_verify(&head, 0, &trailer)).then_some(head)
}

/// Keep the record of `names`, the packs this install of `pin` placed. A
/// record that cannot be kept costs only the long way next time.
pub fn keep(pin: &[u8; 32], names: &[String]) {
    let packs: Option<Vec<Placed>> = names
        .iter()
        .map(|n| Some(Placed { name: n.clone(), head: *blake3::hash(&head(n)?).as_bytes() }))
        .collect();
    let Some(raw) = packs.and_then(|p| encode(pin, &p)) else { return };
    let Some(trailer) = mint(&raw) else { return };
    let pid = mk_getpid();
    let path = at(pin);
    if vfs::write_file(pid, &path, &raw).is_ok() {
        let _ = vfs::write_file(pid, &trailer_of(&path), &trailer);
    }
}

/// The packs, one of them under `pkg`'s own name, when `pkg`'s install of
/// `pin` is already in place; None otherwise.
pub fn already(pkg: &str, pin: &[u8; 32]) -> Option<Vec<String>> {
    let (pid, path) = (mk_getpid(), at(pin));
    let raw = vfs::read_file(pid, &path, MAX_BYTES as u32).ok()?;
    let trailer = vfs::read_file(pid, &trailer_of(&path), MAX_TRAILER).ok()?;
    if !mk_local_verify(&raw, 0, &trailer) {
        return None;
    }
    let packs = decode(&raw, pin)?;
    if !packs.iter().any(|p| p.name == pkg) {
        return None;
    }
    let held = packs.iter().all(|p| head(&p.name).is_some_and(|h| *blake3::hash(&h).as_bytes() == p.head));
    held.then(|| packs.into_iter().map(|p| p.name).collect())
}
