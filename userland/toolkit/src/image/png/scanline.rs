use crate::image::types::DecodeError;

use super::paeth::paeth;

/* Reverse one scanline's filter in place. `prev` is the previous
 * reconstructed row of the same pass, all zero for a pass's first row, and
 * `bpp` the filter unit: bytes per pixel rounded up to at least one, so
 * left and up-left references land on the matching byte for every format,
 * sub-byte packings included. Each filter runs its own loop. */
pub(super) fn unfilter_row(
    filter: u8,
    row: &mut [u8],
    prev: &[u8],
    bpp: usize,
) -> Result<(), DecodeError> {
    let n = row.len();
    let prev = prev.get(..n).ok_or(DecodeError::OutputTooSmall)?;
    let left = |row: &[u8], i: usize| if i >= bpp { row[i - bpp] } else { 0 };
    match filter {
        0 => {}
        1 => (bpp..n).for_each(|i| row[i] = row[i].wrapping_add(row[i - bpp])),
        2 => row.iter_mut().zip(prev).for_each(|(r, p)| *r = r.wrapping_add(*p)),
        3 => (0..n).for_each(|i| {
            let avg = (left(row, i) as u16 + prev[i] as u16) / 2;
            row[i] = row[i].wrapping_add(avg as u8);
        }),
        4 => (0..n).for_each(|i| {
            let p = paeth(left(row, i) as i32, prev[i] as i32, left(prev, i) as i32);
            row[i] = row[i].wrapping_add(p as u8);
        }),
        _ => return Err(DecodeError::Unsupported),
    }
    Ok(())
}
