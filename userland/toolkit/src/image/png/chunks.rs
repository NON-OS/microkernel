use crate::image::types::DecodeError;

use super::be_u32::be_u32;
use super::crc::crc32;

/* Palette and its transparency, borrowed from the input, and the offset of
 * the first IDAT chunk (0 when there is none). The compressed data is read
 * in place, never copied. */
pub(super) struct Chunks<'a> {
    pub plte: &'a [u8],
    pub trns: &'a [u8],
    pub idat_at: usize,
}

/* One chunk: its type and data, and the offset of the chunk that follows. */
pub(super) fn chunk_at(input: &[u8], off: usize) -> Result<(&[u8], &[u8], usize), DecodeError> {
    let clen = be_u32(input, off)? as usize;
    let tag = input.get(off + 4..off + 8).ok_or(DecodeError::Truncated)?;
    let end = off.checked_add(8).and_then(|v| v.checked_add(clen)).ok_or(DecodeError::Truncated)?;
    let data = input.get(off + 8..end).ok_or(DecodeError::Truncated)?;
    Ok((tag, data, end + 4))
}

/* Walk the chunk stream from IHDR to IEND (or the end of input). The chunks
 * the image is built from (IHDR, PLTE, tRNS, IDAT) must carry a correct
 * CRC-32, so a corrupted file is refused rather than drawn from damaged data. */
pub(super) fn gather(input: &[u8]) -> Result<Chunks<'_>, DecodeError> {
    let mut out = Chunks { plte: &[], trns: &[], idat_at: 0 };
    let mut off = 8;
    while off + 8 <= input.len() {
        let (tag, data, next) = chunk_at(input, off)?;
        if matches!(tag, b"IHDR" | b"PLTE" | b"tRNS" | b"IDAT") {
            let crc = be_u32(input, next - 4)?;
            if crc32(&input[off + 4..next - 4]) != crc {
                return Err(DecodeError::BadMagic);
            }
        }
        match tag {
            b"IEND" => break,
            b"IDAT" if out.idat_at == 0 => out.idat_at = off,
            b"PLTE" => out.plte = data,
            b"tRNS" => out.trns = data,
            _ => {}
        }
        off = next;
    }
    Ok(out)
}
