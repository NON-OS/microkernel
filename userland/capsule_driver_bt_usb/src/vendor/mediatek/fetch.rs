/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A package file, read whole from the kernel and checked before an octet
//! of it is used: its length, read one octet past the pin so a longer file
//! shows, then its SHA-256. Only the copy checked is ever sent.

use alloc::vec;
use alloc::vec::Vec;

use super::fail::Fail;
use super::files::File;
use super::port::Port;

/// One read's most; the kernel caps a call at 1 MiB.
const STEP: usize = 64 * 1024;

pub fn fetch(p: &mut impl Port, f: &File) -> Result<Vec<u8>, Fail> {
    let mut buf = vec![0u8; f.len + 1];
    let mut at = 0;
    while at < buf.len() {
        let end = buf.len().min(at + STEP);
        match p.file(f.kind, at as u64, &mut buf[at..end]) {
            n if n < 0 && at == 0 => return Err(Fail::Missing),
            n if n <= 0 => break,
            n => at += (n as usize).min(end - at),
        }
    }
    if at != f.len {
        return Err(Fail::Mismatch);
    }
    buf.truncate(at);
    match p.sha256(&buf) {
        Some(d) if d == f.sha256 => Ok(buf),
        _ => Err(Fail::Mismatch),
    }
}
