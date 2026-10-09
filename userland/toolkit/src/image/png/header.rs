use crate::image::types::{DecodeError, ImageSize};

use super::be_u32::be_u32;
use super::channels::channels;

const PNG_SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

/* IHDR as the decoder uses it. Every color type and bit depth pair the PNG
 * specification allows is accepted, with Adam7 interlacing; 16-bit samples
 * are later reduced to their high byte. */
pub(super) struct Header {
    pub size: ImageSize,
    pub color_type: u8,
    pub channels: usize,
    pub bit_depth: u8,
    pub interlaced: bool,
}

impl Header {
    /* Bits one pixel occupies in a scanline. */
    pub fn bits_pp(&self) -> usize {
        self.channels * self.bit_depth as usize
    }

    /* Packed scanline bytes for `px` pixels, without the filter byte. */
    pub fn row_bytes(&self, px: usize) -> usize {
        px.saturating_mul(self.bits_pp()).saturating_add(7) / 8
    }
}

pub fn png_dimensions(input: &[u8]) -> Result<ImageSize, DecodeError> {
    Ok(parse_header(input)?.size)
}

pub(super) fn parse_header(input: &[u8]) -> Result<Header, DecodeError> {
    if input.get(0..8) != Some(&PNG_SIG) {
        return Err(DecodeError::BadMagic);
    }
    if be_u32(input, 8)? != 13 || input.get(12..16) != Some(b"IHDR") {
        return Err(DecodeError::Unsupported);
    }
    let size = ImageSize::new(be_u32(input, 16)?, be_u32(input, 20)?)?;
    let b = input.get(24..29).ok_or(DecodeError::Truncated)?;
    let (bit_depth, color_type, compression, filter, interlace) = (b[0], b[1], b[2], b[3], b[4]);
    let depth_ok = match color_type {
        0 => matches!(bit_depth, 1 | 2 | 4 | 8 | 16),
        3 => matches!(bit_depth, 1 | 2 | 4 | 8),
        2 | 4 | 6 => matches!(bit_depth, 8 | 16),
        _ => false,
    };
    if !depth_ok || compression != 0 || filter != 0 || interlace > 1 {
        return Err(DecodeError::Unsupported);
    }
    let channels = channels(color_type)?;
    Ok(Header { size, color_type, channels, bit_depth, interlaced: interlace == 1 })
}
