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

//! Bytes over a link a slice at a time. Through a proxy each read or write
//! asks it once, for a slice; a direct socket's receive is the short call
//! it always was, and its send the bounded one.

use super::super::link::Link;
use super::SLICE_MS;

/// What one slice read: how many bytes, and whether the far end has said
/// it finished (a direct socket never says).
pub struct Read {
    pub n: usize,
    pub ended: bool,
}

impl Link {
    /// Carry the front of `data`; how many bytes went. Zero means offer the
    /// same bytes again on a later slice.
    pub fn write_slice(&mut self, data: &[u8]) -> Result<usize, ()> {
        match self {
            Link::Direct { sockets, handle } => {
                /* One call frame at a time; the caller offers the rest next. */
                let piece = super::super::send_cap::front(data);
                super::super::socket_send::socket_send(*sockets, *handle, piece)?;
                Ok(piece.len())
            }
            Link::Routed { stream, .. } => stream.write_slice(data, SLICE_MS).map_err(|_| ()),
        }
    }

    pub fn read_slice(&mut self, into: &mut [u8]) -> Result<Read, ()> {
        match self {
            Link::Direct { sockets, handle } => {
                /* An empty receive is not an error to a direct socket. */
                let n =
                    super::super::socket_recv::socket_recv(*sockets, *handle, into).unwrap_or(0);
                Ok(Read { n, ended: false })
            }
            Link::Routed { stream, .. } => {
                let n = stream.read_slice(into, SLICE_MS).map_err(|_| ())?;
                Ok(Read { n, ended: n == 0 && stream.ended() })
            }
        }
    }
}
