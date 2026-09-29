use crate::image::types::DecodeError;

use super::header::Header;
use super::palette::Palette;
use super::pixel::pixel;

/* Expand one unfiltered row of `n` pixels into ARGB8888, pixel k landing at
 * out[at + k * step]: step 1 for a plain row, the pass spacing for Adam7.
 * Plain 8-bit RGBA and unkeyed RGB rows, the common web cases, convert a
 * whole pixel at a time; every other layout goes sample by sample. */
pub(super) fn row_to_argb(
    h: &Header,
    pal: &Palette<'_>,
    row: &[u8],
    n: usize,
    out: &mut [u32],
    at: usize,
    step: usize,
) -> Result<(), DecodeError> {
    let end = at + (n.max(1) - 1) * step + 1;
    let dst = out.get_mut(at..end).ok_or(DecodeError::OutputTooSmall)?;
    let whole = h.bit_depth == 8 && step == 1 && row.len() >= n * h.channels;
    let rgb = |p: &[u8]| (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32;
    match h.color_type {
        6 if whole => dst.iter_mut().zip(row.chunks_exact(4)).for_each(|(d, p)| {
            *d = (p[3] as u32) << 24 | rgb(p);
        }),
        2 if whole && pal.key[0].is_none() => {
            dst.iter_mut().zip(row.chunks_exact(3)).for_each(|(d, p)| *d = 0xff00_0000 | rgb(p))
        }
        _ => {
            for k in 0..n {
                dst[k * step] = pixel(h, pal, row, k)?;
            }
        }
    }
    Ok(())
}
