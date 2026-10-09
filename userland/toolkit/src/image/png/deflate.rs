use crate::image::types::DecodeError;

/* Where a streaming inflate pulls its compressed bytes from; None ends the
 * stream. */
pub trait ByteSource {
    fn next_byte(&mut self) -> Option<u8>;
}

/* A plain byte slice is a complete compressed stream. */
impl ByteSource for core::slice::Iter<'_, u8> {
    fn next_byte(&mut self) -> Option<u8> {
        self.next().copied()
    }
}

/* LSB-first DEFLATE bit reader over a byte source, buffering up to 64 bits
 * so a Huffman code can be looked at before it is consumed. */
pub struct BitReader<S: ByteSource> {
    src: S,
    buf: u64,
    have: u32,
}

impl<S: ByteSource> BitReader<S> {
    pub const fn new(src: S) -> Self {
        Self { src, buf: 0, have: 0 }
    }

    fn fill(&mut self, want: u32) {
        while self.have < want && self.have <= 56 {
            let Some(b) = self.src.next_byte() else { return };
            self.buf |= (b as u64) << self.have;
            self.have += 8;
        }
    }

    /* Up to 16 upcoming bits without consuming them, and how many are real. */
    pub fn peek16(&mut self) -> (u32, u32) {
        self.fill(16);
        ((self.buf & 0xffff) as u32, self.have.min(16))
    }

    pub fn consume(&mut self, count: u32) {
        let n = count.min(self.have);
        self.buf >>= n;
        self.have -= n;
    }

    pub fn read_bits(&mut self, count: u8) -> Result<u16, DecodeError> {
        let count = count as u32;
        self.fill(count);
        if self.have < count {
            return Err(DecodeError::Truncated);
        }
        let v = (self.buf & ((1u64 << count) - 1)) as u16;
        self.consume(count);
        Ok(v)
    }

    /* Drop the bits left in the current byte, as a stored block requires. */
    pub fn align_byte(&mut self) {
        self.consume(self.have % 8);
    }
}
