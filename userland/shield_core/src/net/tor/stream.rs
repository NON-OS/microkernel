// NONOS Operating System (AGPL-3.0-or-later)
//! One stream over the chosen anonymity network.

use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

#[path = "stream_io.rs"]
mod io;
#[path = "stream_open.rs"]
mod open;

pub(super) const STALL: Duration = Duration::from_secs(30);

#[cfg(target_vendor = "nonos")]
use nonos_route_link::RouteStream;

pub struct TorStream {
    #[cfg(target_vendor = "nonos")]
    inner: RouteStream,
    pub(super) patience: Duration,
}

impl TorStream {
    /// What has arrived within `wait`, zero when nothing did.
    #[cfg(target_vendor = "nonos")]
    pub(super) fn read_within(&mut self, buf: &mut [u8], wait: Duration) -> Result<usize> {
        let ms = u64::try_from(wait.as_millis()).unwrap_or(u64::MAX);
        self.inner.read_wait(buf, ms).map_err(|why| Error::new(ErrorKind::Other, why))
    }

    #[cfg(not(target_vendor = "nonos"))]
    pub(super) fn read_within(&mut self, _buf: &mut [u8], _wait: Duration) -> Result<usize> {
        Err(Error::new(ErrorKind::NotConnected, "no network"))
    }

    /// Whether the far end finished and everything it sent has been read.
    #[cfg(target_vendor = "nonos")]
    fn ended(&self) -> bool {
        self.inner.ended()
    }

    #[cfg(not(target_vendor = "nonos"))]
    fn ended(&self) -> bool {
        false
    }

    #[cfg(target_vendor = "nonos")]
    fn send(&mut self, buf: &[u8]) -> Result<()> {
        self.inner.write_all(buf).map_err(|why| Error::new(ErrorKind::Other, why))
    }

    #[cfg(not(target_vendor = "nonos"))]
    fn send(&mut self, _buf: &[u8]) -> Result<()> {
        Err(Error::new(ErrorKind::NotConnected, "no network"))
    }
}
