use crate::image::gif::lzw;
use crate::image::types::{DecodeError, ImageSize};

use super::frame::Frame;
use super::header::parse_screen;
use super::sub_blocks::{skip, SubBlocks};
use super::to_argb::Canvas;

/* Decode the first frame of a GIF87a/89a into ARGB8888. Extensions are walked
 * for a Graphic Control Extension (transparency); the first image descriptor
 * is clipped to the logical screen and LZW-decoded straight onto the cleared
 * screen, so the work is bounded by the rows that show, not the frame size. */
pub fn decode_gif_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    let screen = parse_screen(input)?;
    let sw = screen.size.width as usize;
    let sh = screen.size.height as usize;
    let count = sw.saturating_mul(sh);
    if count > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    out[..count].fill(0);
    let mut off = screen.blocks_at;
    let mut transparent: Option<u8> = None;
    loop {
        let intro = *input.get(off).ok_or(DecodeError::Truncated)?;
        off += 1;
        if intro == 0x21 {
            let label = *input.get(off).ok_or(DecodeError::Truncated)?;
            off += 1;
            let data = skip(input, &mut off)?;
            if label == 0xF9 && data.len() >= 4 && data[0] & 0x01 != 0 {
                transparent = Some(data[3]);
            }
            continue;
        }
        if intro != 0x2C {
            return Err(DecodeError::Unsupported);
        }
        let d = input.get(off..off + 9).ok_or(DecodeError::Truncated)?;
        let le = |i: usize| u16::from_le_bytes([d[i], d[i + 1]]) as usize;
        let (left, top, fw, fh, packed) = (le(0), le(2), le(4), le(6), d[8]);
        off += 9;
        let lct = if packed & 0x80 != 0 { 3usize << ((packed & 0x07) + 1) } else { 0 };
        let palette: &[u8] = if lct != 0 {
            let local = input.get(off..off + lct).ok_or(DecodeError::Truncated)?;
            off += lct;
            local
        } else {
            &screen.gct
        };
        if palette.is_empty() || fw == 0 || fh == 0 {
            return Err(DecodeError::Unsupported);
        }
        let min_code_size = *input.get(off).ok_or(DecodeError::Truncated)?;
        let frame = Frame::new(fw, fh, left, top, packed & 0x40 != 0);
        /* Frame rows that land on the screen; none when it lies off to the side. */
        let visible = if left < sw { fh.min(sh.saturating_sub(top)) } else { 0 };
        let max_out = fw.saturating_mul(frame.rows_needed(visible));
        let mut canvas = Canvas::new(out, palette, transparent, &frame, (sw, sh), visible);
        lzw::decode(min_code_size, SubBlocks::new(input, off + 1), max_out, &mut canvas)?;
        if !canvas.done() {
            return Err(DecodeError::Truncated);
        }
        return Ok(screen.size);
    }
}
