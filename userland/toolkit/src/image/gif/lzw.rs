use alloc::vec::Vec;

use crate::image::types::DecodeError;

use super::lzw_dict::{Dict, MAX_CODES};
use super::sub_blocks::SubBlocks;
use super::to_argb::Canvas;

/* Decode GIF variable-width LZW straight onto the canvas. `min_code_size`
 * sets the initial code width; codes grow as the table fills and reset on
 * the clear code. Decoding stops at the end code, when the data runs out,
 * after `max_out` indices, or once the canvas has every visible row, so a
 * frame far larger than the screen costs no more than what shows. */
pub(super) fn decode(
    min_code_size: u8,
    mut data: SubBlocks<'_>,
    max_out: usize,
    canvas: &mut Canvas<'_>,
) -> Result<(), DecodeError> {
    if !(2..=8).contains(&min_code_size) {
        return Err(DecodeError::Unsupported);
    }
    let clear = 1u16 << min_code_size;
    let mut dict = Dict::new(clear);
    let mut code_size = min_code_size + 1;
    let mut old: Option<u16> = None;
    let mut first: u8 = 0;
    let mut stack: Vec<u8> = Vec::with_capacity(MAX_CODES + 1);
    let (mut buf, mut bits, mut emitted) = (0u32, 0u32, 0usize);
    while emitted < max_out && !canvas.done() {
        while bits < code_size as u32 {
            let Some(byte) = data.next_byte() else { return Ok(()) };
            buf |= (byte as u32) << bits;
            bits += 8;
        }
        let code = (buf & ((1u32 << code_size) - 1)) as u16;
        buf >>= code_size;
        bits -= code_size as u32;
        if code == clear {
            (code_size, dict.free, old) = (min_code_size + 1, clear + 2, None);
            continue;
        }
        if code == clear + 1 {
            return Ok(());
        }
        let Some(prev) = old else {
            first = dict.root(code);
            canvas.put(first)?;
            (emitted, old) = (emitted + 1, Some(code));
            continue;
        };
        first = dict.expand(code, prev, first, &mut stack)?;
        while let Some(b) = stack.pop() {
            if emitted >= max_out || canvas.done() {
                return Ok(());
            }
            canvas.put(b)?;
            emitted += 1;
        }
        dict.add(prev, first);
        if dict.free == (1u16 << code_size) && code_size < 12 {
            code_size += 1;
        }
        old = Some(code);
    }
    Ok(())
}
