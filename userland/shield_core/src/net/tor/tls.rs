// NONOS Operating System (AGPL-3.0-or-later)
//! TLS 1.3 over a stream on the anonymity network, through `nonos_tls`, the
//! one client every NONOS capsule uses: the Mozilla roots, the host checked
//! against the leaf, the chain walked and every signature checked before the
//! request is sealed.
//!
//! The phones' HTTP code writes a whole request, flushes and reads to the
//! end, one request per connection. This stream holds what is written, runs
//! the exchange on the flush, and then reads out the server's plaintext.

use super::TorStream;
use crate::error::NetError;
use std::io::{Error, ErrorKind, Read, Result, Write};

/// The largest response held: a page of pool history.
const LIMIT: usize = 16 * 1024 * 1024;

pub struct TlsStream {
    stream: TorStream,
    host: String,
    request: Vec<u8>,
    reply: Option<Vec<u8>>,
    at: usize,
}

pub fn tls(stream: TorStream, host: &str) -> std::result::Result<TlsStream, NetError> {
    if host.is_empty() {
        return Err(NetError::EndpointRefused);
    }
    Ok(TlsStream { stream, host: host.to_owned(), request: Vec::new(), reply: None, at: 0 })
}

#[cfg(target_vendor = "nonos")]
impl nonos_tls::Io for TorStream {
    fn write_all(&mut self, data: &[u8]) -> core::result::Result<(), nonos_tls::SessionError> {
        Write::write_all(self, data).map_err(|_| nonos_tls::SessionError::Io)
    }
    fn read(&mut self, into: &mut [u8]) -> core::result::Result<usize, nonos_tls::SessionError> {
        let wait = self.patience;
        self.read_within(into, wait).map_err(|_| nonos_tls::SessionError::Io)
    }
}

impl TlsStream {
    #[cfg(target_vendor = "nonos")]
    fn run(&mut self) -> Result<()> {
        let now = nonos_tls::rtc_now();
        if now == 0 {
            return Err(Error::new(ErrorKind::Other, "no clock to check the certificate against"));
        }
        let plain = nonos_tls::exchange(&mut self.stream, &self.host, &self.request, now, LIMIT)
            .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "tls"))?;
        self.reply = Some(plain);
        Ok(())
    }

    #[cfg(not(target_vendor = "nonos"))]
    fn run(&mut self) -> Result<()> {
        let _ = (&self.stream, &self.host, LIMIT);
        Err(Error::new(ErrorKind::NotConnected, "no network"))
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.reply.is_some() {
            return Err(Error::new(ErrorKind::Other, "one request per connection"));
        }
        self.request.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        if self.reply.is_none() {
            self.run()?;
        }
        Ok(())
    }
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.reply.is_none() {
            self.run()?;
        }
        let reply = self.reply.as_deref().unwrap_or(&[]);
        let rest = reply.get(self.at..).unwrap_or(&[]);
        let n = rest.len().min(buf.len());
        buf[..n].copy_from_slice(&rest[..n]);
        self.at += n;
        Ok(n)
    }
}
