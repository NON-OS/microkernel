// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! One boot console line for every Intel Wi-Fi function discovery sees, run
//! or not: its id, its generation and whether this driver boots it. A card
//! with no boot path here is otherwise silent, and a laptop's photo of the
//! log is how the owner learns which card it carries.

use nonos_libc::mk_debug;

use crate::firmware::generation::name;

const LINE_MAX: usize = 200;

struct Line {
    buf: [u8; LINE_MAX],
    len: usize,
}

impl Line {
    fn put(&mut self, t: &[u8]) {
        let n = t.len().min(LINE_MAX - 1 - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&t[..n]);
        self.len += n;
    }
}

fn hex4(v: u16) -> [u8; 4] {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    core::array::from_fn(|i| DIGITS[usize::from((v >> (12 - i * 4)) & 0xF)])
}

/// Say what the Intel Wi-Fi function with PCI device id `device` is.
pub fn announce(device: u16) {
    let mut l = Line { buf: [0; LINE_MAX], len: 0 };
    l.put(b"[iwlwifi] pci 8086:");
    l.put(&hex4(device));
    match name(device) {
        Some(g) if g.boots => {
            l.put(b": ");
            l.put(g.what.as_bytes());
        }
        Some(g) => {
            l.put(b": ");
            l.put(g.what.as_bytes());
            l.put(b": no boot path in this driver (Linux runs ");
            l.put(g.linux_fw.as_bytes());
            l.put(b")");
        }
        None => l.put(b": an Intel Wi-Fi id this driver does not know"),
    }
    l.buf[l.len] = b'\n';
    let _ = mk_debug(l.buf.as_ptr(), l.len + 1);
}
