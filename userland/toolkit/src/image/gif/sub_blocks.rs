use crate::image::types::DecodeError;

/* A GIF sub-block chain read in place: each block is a length byte and that
 * many bytes, the chain ends at a zero length. Bytes come out one at a time
 * across block boundaries, so the compressed data is never gathered. */
pub(super) struct SubBlocks<'a> {
    input: &'a [u8],
    pos: usize,
    left: usize,
    ended: bool,
}

impl<'a> SubBlocks<'a> {
    pub fn new(input: &'a [u8], at: usize) -> Self {
        Self { input, pos: at, left: 0, ended: false }
    }

    /* The next data byte; None at the terminator or where the input stops. */
    pub fn next_byte(&mut self) -> Option<u8> {
        while self.left == 0 {
            if self.ended {
                return None;
            }
            let len = *self.input.get(self.pos)? as usize;
            self.pos += 1;
            self.left = len;
            self.ended = len == 0;
        }
        let b = *self.input.get(self.pos)?;
        self.pos += 1;
        self.left -= 1;
        Some(b)
    }
}

/* Skip the chain starting at `*off`, leaving `*off` past its terminator, and
 * return its first block (the whole payload of a Graphic Control Extension). */
pub(super) fn skip<'a>(input: &'a [u8], off: &mut usize) -> Result<&'a [u8], DecodeError> {
    let mut first: &[u8] = &[];
    loop {
        let len = *input.get(*off).ok_or(DecodeError::Truncated)? as usize;
        *off += 1;
        if len == 0 {
            return Ok(first);
        }
        let data = input.get(*off..*off + len).ok_or(DecodeError::Truncated)?;
        if first.is_empty() {
            first = data;
        }
        *off += len;
    }
}
