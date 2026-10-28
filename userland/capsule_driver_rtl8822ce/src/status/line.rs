// NONOS Operating System (AGPL-3.0-or-later)

//! The console line, assembled whole before it is written. The shared files
//! say a line in pieces (a text, a number, a newline) and under the RTL8821CE's
//! tag; this collects the pieces, puts this driver's tag in place of that one
//! and hands the finished line on in one write. Pure, so it is held on the host.

/// The longest line written; a longer one is cut and still ends in a newline.
pub const LINE_MAX: usize = 160;
/// The tag the shared files' lines start with, and this driver's in its place.
pub const SHARED_TAG: &[u8] = b"[rtl8821ce]";
pub const TAG: &[u8] = b"[rtl8822ce]";

const _: () = assert!(SHARED_TAG.len() == TAG.len());

/// A line being assembled.
pub struct Line {
    buf: [u8; LINE_MAX],
    len: usize,
}

impl Line {
    /// A line resumed from what an earlier piece left: `buf`, of which the
    /// first `len` bytes are held.
    pub fn resume(buf: [u8; LINE_MAX], len: usize) -> Self {
        Self { buf, len: len.min(LINE_MAX - 1) }
    }

    /// What is held so far, to keep between pieces.
    pub fn held(&self) -> (&[u8; LINE_MAX], usize) {
        (&self.buf, self.len)
    }

    /// Take `piece`, handing `write` each line it completes, whole.
    pub fn push(&mut self, piece: &[u8], mut write: impl FnMut(&[u8])) {
        for &b in piece {
            if b == b'\n' {
                self.end(&mut write);
                continue;
            }
            // Room is kept for the newline the cut line still ends in.
            if self.len < LINE_MAX - 1 {
                self.buf[self.len] = b;
                self.len += 1;
            }
        }
    }

    fn end(&mut self, write: &mut impl FnMut(&[u8])) {
        if self.buf[..self.len].starts_with(SHARED_TAG) {
            self.buf[..TAG.len()].copy_from_slice(TAG);
        }
        self.buf[self.len] = b'\n';
        write(&self.buf[..=self.len]);
        self.len = 0;
    }
}
