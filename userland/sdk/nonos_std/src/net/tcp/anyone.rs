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

//! A stream over the Anyone network, through the shared route client's
//! tunnel to net.anon. net.anon keys that tunnel on the app, so an app holds
//! one Anyone stream at a time: a second one is refused while the first is
//! open, rather than ending the first behind its back.

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_route_link::{Route, RouteStream};

use crate::io::{Error, ErrorKind, Result};

/// How long one read waits for the far end before looking again.
const READ_WAIT_MS: u64 = 1000;

static HELD: AtomicBool = AtomicBool::new(false);

pub(crate) struct AnyoneStream {
    stream: RouteStream,
}

impl AnyoneStream {
    pub(crate) fn open(route: Route, host: &str, port: u16) -> Result<Self> {
        if HELD.swap(true, Ordering::AcqRel) {
            return Err(Error::new(
                ErrorKind::Other,
                "an app holds one stream over the Anyone network at a time",
            ));
        }
        match RouteStream::connect(route, host, port) {
            Ok(stream) => Ok(Self { stream }),
            Err(why) => {
                HELD.store(false, Ordering::Release);
                Err(Error::new(ErrorKind::Other, why))
            }
        }
    }

    /// What has arrived, waiting for the first of it; zero once the far end
    /// has finished and everything it sent has been read.
    pub(crate) fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let n = self
                .stream
                .read_wait(buf, READ_WAIT_MS)
                .map_err(|why| Error::new(ErrorKind::Other, why))?;
            if n > 0 || self.stream.ended() {
                return Ok(n);
            }
        }
    }

    pub(crate) fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write_all(buf).map_err(|why| Error::new(ErrorKind::Other, why))?;
        Ok(buf.len())
    }
}

impl Drop for AnyoneStream {
    fn drop(&mut self) {
        HELD.store(false, Ordering::Release);
    }
}
