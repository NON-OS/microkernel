use crate::image::types::DecodeError;

mod decode;
mod header;
mod le;
mod masks;

pub use decode::decode_bmp_argb8888;
pub use header::bmp_dimensions;

/* The BMP forms this decoder reads: a BITMAPINFOHEADER or later (V2 to V5)
 * DIB header with BI_RGB at 1, 4, 8, 16, 24 or 32 bits per pixel, or
 * BI_BITFIELDS / BI_ALPHABITFIELDS at 16 or 32. OS/2 core headers, RLE and
 * embedded JPEG or PNG are Unsupported. */
fn readable(dib: usize, compression: u32, bpp: usize) -> Result<(), DecodeError> {
    match (dib >= 40, compression, bpp) {
        (true, 0, 1 | 4 | 8 | 16 | 24 | 32) | (true, 3 | 6, 16 | 32) => Ok(()),
        _ => Err(DecodeError::Unsupported),
    }
}
