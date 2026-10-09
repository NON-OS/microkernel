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
//! HTTP/1.1 for a client, with no I/O of its own.

//! A chunked body, decoded as it arrives, so a file of any size is never held
//! whole.

use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum ChunkError {
    /// A size line that is not hexadecimal, or one too long to be one.
    BadSize,
    /// The CRLF after a chunk's data was not there.
    BadEnd,
}

enum State {
    /// Reading a size line; the hex digits so far.
    Size(Vec<u8>),
    /// Data bytes still to come in this chunk.
    Data(u64),
    /// The CRLF after a chunk's data; how many of its two bytes seen.
    DataEnd(u8),
    /// The last chunk came; trailers are read and dropped.
    Trailer(Vec<u8>),
    Done,
}

pub struct Chunked {
    state: State,
}

impl Default for Chunked {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunked {
    pub const fn new() -> Self {
        Self { state: State::Size(Vec::new()) }
    }

    /// Whether the last chunk and its trailers have come.
    pub fn done(&self) -> bool {
        matches!(self.state, State::Done)
    }

    /// Feed `input`; the body bytes in it are appended to `out`.
    pub fn feed(&mut self, mut input: &[u8], out: &mut Vec<u8>) -> Result<(), ChunkError> {
        while !input.is_empty() {
            match &mut self.state {
                State::Size(line) => {
                    let b = input[0];
                    input = &input[1..];
                    if b == b'\n' {
                        let text = core::str::from_utf8(line).map_err(|_| ChunkError::BadSize)?;
                        let digits = text.trim_end_matches('\r').split(';').next().unwrap_or("").trim();
                        if digits.is_empty() || digits.len() > 15 {
                            return Err(ChunkError::BadSize);
                        }
                        let n = u64::from_str_radix(digits, 16).map_err(|_| ChunkError::BadSize)?;
                        self.state = if n == 0 { State::Trailer(Vec::new()) } else { State::Data(n) };
                    } else {
                        if line.len() >= 128 {
                            return Err(ChunkError::BadSize);
                        }
                        line.push(b);
                    }
                }
                State::Data(left) => {
                    let take = (*left).min(input.len() as u64) as usize;
                    out.extend_from_slice(&input[..take]);
                    input = &input[take..];
                    *left -= take as u64;
                    if *left == 0 {
                        self.state = State::DataEnd(0);
                    }
                }
                State::DataEnd(seen) => {
                    let want = if *seen == 0 { b'\r' } else { b'\n' };
                    if input[0] != want {
                        return Err(ChunkError::BadEnd);
                    }
                    input = &input[1..];
                    *seen += 1;
                    if *seen == 2 {
                        self.state = State::Size(Vec::new());
                    }
                }
                State::Trailer(line) => {
                    let b = input[0];
                    input = &input[1..];
                    if b == b'\n' {
                        let blank = line.is_empty() || line.as_slice() == b"\r";
                        line.clear();
                        if blank {
                            self.state = State::Done;
                        }
                    } else if line.len() < 1024 {
                        line.push(b);
                    }
                }
                State::Done => return Ok(()),
            }
        }
        Ok(())
    }
}
