// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! One line at a time to the boot serial, through MkDebug when the build
//! grants it; a build without serial debug drops the line, which is the
//! capsule's only output it can lose.

use super::line_frame::framed;
use alloc::vec::Vec;
use nonos_libc::mk_debug;

/// The kernel takes 256 bytes a call; a longer line is cut.
const MAX: usize = 256;

pub struct Line(Vec<u8>);

impl Line {
    pub fn new(prefix: &[u8]) -> Self {
        let mut v = Vec::with_capacity(96);
        v.extend_from_slice(b"[ACPI] ");
        v.extend_from_slice(prefix);
        Line(v)
    }

    /// A line under another tag than `[ACPI]`, as `[SMBIOS] `.
    pub fn tagged(tag: &[u8]) -> Self {
        Line(tag.to_vec())
    }

    pub fn str(mut self, s: &[u8]) -> Self {
        self.0.extend_from_slice(s);
        self
    }

    pub fn hex(mut self, v: u64) -> Self {
        let digits = (64 - v.leading_zeros()).div_ceil(4).max(1);
        self.0.extend_from_slice(b"0x");
        for i in (0..digits).rev() {
            self.0.push(b"0123456789ABCDEF"[((v >> (i * 4)) & 0xF) as usize]);
        }
        self
    }

    pub fn dec(mut self, v: u64) -> Self {
        let mut digits = [0u8; 20];
        let (mut n, mut i) = (v, 20);
        loop {
            i -= 1;
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        self.0.extend_from_slice(&digits[i..]);
        self
    }

    pub fn end(self) {
        let line = framed(self.0, MAX);
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}
