// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::color::model;
use super::emit::emit;
use super::marker::{next_marker, resume_at};
use super::state::State;
use super::walk::walk;
use crate::image::jpeg::bits::BitReader;
use crate::image::jpeg::coef::refuse::MALFORMED;
use crate::image::types::DecodeError;

/// Decode a baseline, extended or progressive JPEG at 1/2^`shift` of its
/// size (shift 0..=3, an IDCT that reads fewer coefficients), handing each
/// ARGB row to `row`. Coefficient storage above `max_coef` bytes is refused
/// before it is allocated. A stream that ends after at least one scan
/// yields the picture decoded so far. Returns the output size.
pub fn decode_scaled(
    input: &[u8],
    shift: u32,
    max_coef: usize,
    row: &mut dyn FnMut(usize, &[u32]),
) -> Result<(u32, u32), DecodeError> {
    if input.len() < 4 || input[..2] != [0xFF, 0xD8] || shift > 3 {
        return Err(DecodeError::BadMagic);
    }
    let mut st = State::new(8 >> shift, max_coef);
    let (mut pos, mut scans) = (2, 0);
    while let Some((m, p)) = next_marker(input, pos) {
        pos = p;
        if matches!(m, 0xD8 | 0x01) {
            continue;
        }
        if m == 0xD9 {
            break;
        }
        let seg = segment(input, &mut pos)?;
        if m != 0xDA {
            st.header(m, seg)?;
            continue;
        }
        let scan = st.begin_scan(seg)?;
        let f = st.frame.as_ref().ok_or(MALFORMED)?;
        let mut br = BitReader::new(input, pos);
        walk(f, &scan, &mut st.planes, (&st.dc, &st.ac), st.ri, &mut br)?;
        pos = resume_at(&br);
        scans += 1;
    }
    let f = st.frame.as_ref().filter(|_| scans > 0).ok_or(DecodeError::Truncated)?;
    let ids = [0, 1, 2, 3].map(|i| f.comps[i].id);
    emit(f, &st.planes, &st.qnat, model(f.n, st.adobe, ids), row)
}

/// A marker segment's body, `pos` moved past it.
fn segment<'a>(input: &'a [u8], pos: &mut usize) -> Result<&'a [u8], DecodeError> {
    let len = input.get(*pos..*pos + 2).ok_or(DecodeError::Truncated)?;
    let len = u16::from_be_bytes([len[0], len[1]]) as usize;
    let body = input.get(*pos + 2..*pos + len.max(2)).ok_or(DecodeError::Truncated)?;
    *pos += len.max(2);
    Ok(body)
}
