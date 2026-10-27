/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The driver's console lines. Each is built whole in one buffer and
//! written with one call, so no other process's output lands inside it.
//! A line names steps and numbers only: never an address, a network's
//! name or a serial.

const CAP: usize = 120;
const PREFIX: &[u8] = b"[mt7921] ";

pub struct Line {
    buf: [u8; CAP],
    len: usize,
}

impl Line {
    pub fn new(text: &str) -> Line {
        let mut l = Line { buf: [0; CAP], len: 0 };
        l.put(PREFIX);
        l.text(text)
    }

    /// Bytes past the line's room are dropped; the newline always fits.
    fn put(&mut self, b: &[u8]) {
        let n = b.len().min(CAP - 1 - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&b[..n]);
        self.len += n;
    }

    pub fn text(mut self, t: &str) -> Line {
        self.put(t.as_bytes());
        self
    }

    pub fn dec(mut self, mut v: u64) -> Line {
        let mut d = [0u8; 20];
        let mut i = d.len();
        loop {
            i -= 1;
            d[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        self.put(&d[i..]);
        self
    }

    pub fn hex(mut self, v: u32) -> Line {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut d = *b"0x00000000";
        for (i, c) in d[2..].iter_mut().enumerate() {
            *c = DIGITS[(v >> (28 - 4 * i) & 0xf) as usize];
        }
        self.put(&d);
        self
    }

    pub fn say(mut self) {
        self.buf[self.len] = b'\n';
        let _ = nonos_libc::mk_debug(self.buf.as_ptr(), self.len + 1);
    }
}

pub fn line(text: &str) {
    Line::new(text).say()
}
