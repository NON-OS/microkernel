use crate::image::types::{DecodeError, ImageSize};

use super::chunks::gather;
use super::header::parse_header;
use super::idat::IdatBytes;
use super::inflate::{inflate_zlib, Sink};
use super::palette::Palette;
use super::rows::Rows;

/* Decode a PNG of any standard color type and bit depth, plain or Adam7
 * interlaced, into ARGB8888. `out` must hold at least width*height pixels.
 * The compressed data is read in place from its IDAT chunks and inflated
 * through a 32 KiB window one scanline at a time, so beyond `out` the
 * decoder holds two rows and the window, never the whole inflated image.
 * 16-bit samples keep their high byte; tRNS makes palette entries, or the
 * one keyed grey or RGB value, transparent. */
pub fn decode_png_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    let h = parse_header(input)?;
    let w = h.size.width as usize;
    let ht = h.size.height as usize;
    if w.saturating_mul(ht) > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    let chunks = gather(input)?;
    if chunks.idat_at == 0 {
        return Err(DecodeError::Unsupported);
    }
    let pal = Palette::new(&h, chunks.plte, chunks.trns);
    let mut rows = Rows::new(&h, pal, out);
    inflate_zlib(IdatBytes::new(input, chunks.idat_at)?, &mut rows)?;
    if !rows.done() {
        return Err(DecodeError::Truncated);
    }
    Ok(h.size)
}
