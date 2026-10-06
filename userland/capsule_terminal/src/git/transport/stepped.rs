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

//! The git transport for a job: requests are built as `Https` builds them
//! and left in an `Ask` for the job to carry; nothing here touches the
//! network. A request with no answer yet reads to nonos_git as a closed
//! connection, which ends that run of the step and is never shown.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;

use nonos_git::{Transport, TransportError};
use nonos_http::{RequestBuilder, Url};

use super::ask::Ask;

pub struct Stepped {
    pub remote: Url,
    pub ask: Ask,
}

impl Stepped {
    pub fn new(remote: Url) -> Stepped {
        Stepped { remote, ask: Ask::new() }
    }

    fn carry(&mut self, request: Vec<u8>) -> Result<Vec<u8>, TransportError> {
        self.ask.request(request).ok_or(TransportError::Closed)
    }
}

impl Transport for Stepped {
    fn get(&mut self, path: &str) -> Result<Vec<u8>, TransportError> {
        let target = format!("{}{}", self.remote.path, path);
        let request = RequestBuilder::get(&self.remote.host, &target).build();
        self.carry(request.bytes)
    }

    fn post(
        &mut self,
        path: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        let target = format!("{}{}", self.remote.path, path);
        let request = RequestBuilder::post(&self.remote.host, &target, content_type, body).build();
        self.carry(request.bytes)
    }
}
