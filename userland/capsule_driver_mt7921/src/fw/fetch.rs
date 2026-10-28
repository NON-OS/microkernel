/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One package file, read from the kernel and checked. The whole file is
//! read before a byte of it is used, its length and SHA-256 are compared
//! with the pins, and only the bytes checked are ever downloaded: the copy
//! in the capsule's memory, not a second read.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_install_source;
use sha2::{Digest, Sha256};

use super::package::File;

/// One `MkInstallSource` call's most; the kernel caps a call at 1 MiB.
const STEP: usize = 64 * 1024;

pub enum Fetched {
    /// The kernel holds no such file: the package is not on the medium.
    Missing,
    /// A file is there, but not the one pinned: its length or hash differ.
    Mismatch,
    Bytes(Vec<u8>),
}

/// Read `f` through `read` (`MkInstallSource` in the capsule): `(kind,
/// offset, out)` to bytes written, zero at the end, negative for none.
pub fn fetch_with(f: &File, mut read: impl FnMut(u64, u64, &mut [u8]) -> i64) -> Fetched {
    // One byte past the pinned length, so a longer file shows.
    let mut buf = vec![0u8; f.len + 1];
    let mut at = 0usize;
    while at < buf.len() {
        let end = buf.len().min(at + STEP);
        match read(f.kind, at as u64, &mut buf[at..end]) {
            n if n < 0 && at == 0 => return Fetched::Missing,
            n if n <= 0 => break,
            n => at += (n as usize).min(end - at),
        }
    }
    if at != f.len {
        return Fetched::Mismatch;
    }
    buf.truncate(at);
    let digest: [u8; 32] = Sha256::digest(&buf).into();
    if digest == f.sha256 {
        Fetched::Bytes(buf)
    } else {
        Fetched::Mismatch
    }
}

pub fn fetch(f: &File) -> Fetched {
    fetch_with(f, mk_install_source)
}
