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

//! One LZMA chunk inside LZMA2: its resets, then a fresh range coder over
//! exactly its packed bytes, which must end with the code at zero.

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::lzma_decode::chunk;
use super::lzma_model::Model;
use super::range::Range;

/// What carries from one chunk to the next.
pub struct State {
    /// Where the dictionary was last reset.
    pub start: usize,
    pub fresh: bool,
    pub model: Option<Box<Model>>,
    pub dict: u32,
}

/// The chunk at `at`; the offset after it.
pub fn lzma(d: &[u8], at: usize, st: &mut State, out: &mut Vec<u8>) -> Option<usize> {
    let head = d.get(at..at + 5)?;
    let reset = (head[0] >> 5) & 3;
    if reset == 3 {
        (st.start, st.fresh) = (out.len(), true);
    }
    let unpacked = (usize::from(head[0] & 0x1F) << 16)
        + usize::from(u16::from_be_bytes([head[1], head[2]]))
        + 1;
    let packed = usize::from(u16::from_be_bytes([head[3], head[4]])) + 1;
    let mut at = at + 5;
    if reset >= 2 {
        st.model = Some(Box::new(Model::new(*d.get(at)?)?));
        at += 1;
    } else if reset == 1 {
        st.model.as_mut()?.reset();
    }
    if !st.fresh {
        return None;
    }
    let mut rc = Range::new(d.get(at..at + packed)?)?;
    chunk(&mut rc, st.model.as_mut()?, out, st.start, unpacked, st.dict)?;
    rc.finished().then_some(at + packed)
}
