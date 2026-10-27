// NONOS Operating System (AGPL-3.0-or-later)

//! Bring-up status on the boot console, one whole line per call. A line is
//! assembled in full (`Line`) and written once, so it can never interleave
//! with another writer's half line. No line carries an address, a network
//! name or a serial: only stage names, counts and register status words.

/// The longest line; longer text is cut, never split across two writes.
pub const LINE_MAX: usize = 120;

/// Write `msg`, a complete line ending in `\n`.
pub fn line(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}

/// A line under construction.
pub struct Line {
    buf: [u8; LINE_MAX],
    len: usize,
}

impl Line {
    pub fn new(text: &[u8]) -> Self {
        let mut l = Self { buf: [0; LINE_MAX], len: 0 };
        l.text(text);
        l
    }

    pub fn text(&mut self, text: &[u8]) -> &mut Self {
        let room = LINE_MAX - 1 - self.len;
        let n = text.len().min(room);
        self.buf[self.len..self.len + n].copy_from_slice(&text[..n]);
        self.len += n;
        self
    }

    pub fn dec(&mut self, mut n: u32) -> &mut Self {
        let mut d = [0u8; 10];
        let mut i = d.len();
        loop {
            i -= 1;
            d[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        self.text(&d[i..])
    }

    pub fn hex32(&mut self, v: u32) -> &mut Self {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut d = [0u8; 10];
        d[0] = b'0';
        d[1] = b'x';
        for (i, b) in d[2..].iter_mut().enumerate() {
            *b = DIGITS[((v >> (28 - i * 4)) & 0xF) as usize];
        }
        self.text(&d)
    }

    /// The finished line, `\n` appended.
    pub fn bytes(&mut self) -> &[u8] {
        self.buf[self.len] = b'\n';
        &self.buf[..self.len + 1]
    }

    pub fn emit(&mut self) {
        line(self.bytes());
    }
}
