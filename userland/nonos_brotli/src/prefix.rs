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

use crate::bits::Bits;
use crate::error::Error;
use crate::huff::Code;
use alloc::vec::Vec;

/// The largest alphabet a stream codes: insert-and-copy lengths.
pub(crate) const MAX_ALPHABET: usize = 704;

/// One prefix code over `alphabet` symbols (RFC 7932 3.4 and 3.5).
pub(crate) fn read_code(b: &mut Bits, alphabet: usize) -> Result<Code, Error> {
    match b.read(2)? {
        1 => simple(b, alphabet),
        hskip => crate::complex::read(b, alphabet, hskip as usize),
    }
}

/// Up to four symbols listed outright, with lengths fixed by their count.
fn simple(b: &mut Bits, alphabet: usize) -> Result<Code, Error> {
    let n = b.read(2)? as usize + 1;
    let width = usize::BITS - (alphabet - 1).leading_zeros();
    let mut syms = [0usize; 4];
    for i in 0..n {
        syms[i] = b.read(width)? as usize;
        if syms[i] >= alphabet || syms[..i].contains(&syms[i]) {
            return Err(Error::Invalid);
        }
    }
    let lens: &[u8] = match n {
        1 => return Ok(Code::single(syms[0] as u16)),
        2 => &[1, 1],
        3 => &[1, 2, 2],
        _ if b.read(1)? == 0 => &[2, 2, 2, 2],
        _ => &[1, 2, 3, 3],
    };
    let mut full = [0u8; MAX_ALPHABET];
    syms.iter().zip(lens).for_each(|(&s, &l)| full[s] = l);
    Code::build(&full[..alphabet])
}

/// `n` prefix codes over the same alphabet.
pub(crate) fn read_codes(b: &mut Bits, n: usize, alphabet: usize) -> Result<Vec<Code>, Error> {
    let mut all = Vec::new();
    all.try_reserve_exact(n).map_err(|_| Error::NoMemory)?;
    for _ in 0..n {
        all.push(read_code(b, alphabet)?);
    }
    Ok(all)
}
