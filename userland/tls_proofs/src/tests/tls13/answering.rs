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


use std::vec::Vec;

use crate::session::{Io, SessionError};

/// A server that answers the client's hello with `answer(hello)`.
pub(super) struct Answering<F: FnMut(&[u8]) -> Vec<u8>> {
    pub(super) answer: F,
    pub(super) hello: Option<Vec<u8>>,
    pub(super) queued: Vec<u8>,
}

impl<F: FnMut(&[u8]) -> Vec<u8>> Io for Answering<F> {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        if self.hello.is_none() {
            self.hello = Some(data.to_vec());
            self.queued = (self.answer)(data);
        }
        Ok(())
    }
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        if self.queued.is_empty() {
            return Err(SessionError::Io);
        }
        let n = into.len().min(self.queued.len());
        into[..n].copy_from_slice(&self.queued[..n]);
        self.queued.drain(..n);
        Ok(n)
    }
}
