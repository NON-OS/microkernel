// NONOS Operating System (AGPL-3.0-or-later)

//! The package a proof gives the driver: files made in the proof, pinned
//! to their own hashes, and served (or not) through the `MkInstallSource`
//! slots the loader's modules fill, the same interface the capsule reads.

use std::cell::Cell;

use nonos_libc::set_slot;
use sha2::{Digest, Sha256};

use crate::fw::package::File;
use crate::ids::Chip as ChipId;

thread_local! {
    static FILES: Cell<Option<(&'static File, &'static File)>> = const { Cell::new(None) };
}

/// A file for slot `kind` holding `bytes`, pinned to their own hash.
pub fn file(kind: u64, bytes: &[u8]) -> &'static File {
    let name: &'static str = Box::leak(format!("slot{kind:x}.bin").into_boxed_str());
    let sha256: [u8; 32] = Sha256::digest(bytes).into();
    Box::leak(Box::new(File { name, kind, len: bytes.len(), sha256 }))
}

/// The files `pick` hands the driver: `patch` and `ram`, put in slots
/// 0x24 and 0x25 when `served`, and pinned to their own hashes.
pub fn package(patch: &[u8], ram: &[u8], served: bool) {
    let (p, r) = (file(0x24, patch), file(0x25, ram));
    set_slot(0x24, served.then(|| patch.to_vec()));
    set_slot(0x25, served.then(|| ram.to_vec()));
    FILES.with(|f| f.set(Some((p, r))));
}

/// The bring-up's choice of files, as `bringup::Pick`.
pub fn pick(_: ChipId) -> (&'static File, &'static File) {
    FILES.with(|f| f.get()).expect("a proof sets its package first")
}
