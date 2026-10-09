// NONOS Operating System (AGPL-3.0-or-later)
//! One stream over the chosen anonymity network.

use crate::error::NetError;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::time::{Duration, Instant};

pub(super) const STALL: Duration = Duration::from_secs(30);

#[cfg(target_vendor = "nonos")]
use nonos_route_link::{Route, RouteStream};

/// The chosen route, refused unless it is anonymous and up.
#[cfg(target_vendor = "nonos")]
pub(super) fn anonymous_route() -> std::result::Result<Route, NetError> {
    let route = Route::chosen();
    match route {
        Route::Nym(_) | Route::Anon(_) => Ok(route),
        Route::Direct | Route::Down(_) => Err(NetError::ProxyUnreachable),
    }
}

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn anonymous_route() -> std::result::Result<(), NetError> {
    Err(NetError::ProxyUnreachable)
}

pub struct TorStream {
    #[cfg(target_vendor = "nonos")]
    inner: RouteStream,
    pub(super) patience: Duration,
}

impl TorStream {
    #[cfg(target_vendor = "nonos")]
    pub(super) fn open(host: &str, port: u16, limit: Duration) -> std::result::Result<Self, NetError> {
        let route = anonymous_route()?;
        let inner = RouteStream::connect(route, host, port).map_err(|_| NetError::Transport)?;
        let patience = Duration::from_millis(route.patience_ms()).min(limit).max(Duration::from_secs(1));
        Ok(TorStream { inner, patience })
    }

    #[cfg(not(target_vendor = "nonos"))]
    pub(super) fn open(_host: &str, _port: u16, _limit: Duration) -> std::result::Result<Self, NetError> {
        Err(NetError::ProxyUnreachable)
    }

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

impl Read for TorStream {
    /// Blocks until something arrives, the far end closes, or the stall
    /// limit passes with nothing, as the phones' stream does.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let deadline = Instant::now() + STALL;
        loop {
            let n = self.read_within(buf, self.patience)?;
            if n > 0 {
                return Ok(n);
            }
            /* The far end closed and nothing is left: the end of the stream, as a plain HTTP
             * answer with Connection: close ends. Waiting on would turn it into a timeout. */
            if self.ended() {
                return Ok(0);
            }
            if Instant::now() >= deadline {
                return Err(Error::new(ErrorKind::TimedOut, "no progress"));
            }
        }
    }
}

impl Write for TorStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.send(buf)?;
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
