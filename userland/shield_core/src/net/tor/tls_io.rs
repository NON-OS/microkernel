// NONOS Operating System (AGPL-3.0-or-later)
//! The stream's two halves for the phones' HTTP code: what is written is
//! held until the flush runs the exchange, and the plaintext is read out
//! after it, one request per connection.

use super::TlsStream;
use std::io::{Error, Read, Result, Write};

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.reply.is_some() {
            return Err(Error::other("one request per connection"));
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
        let (Some(into), Some(from)) = (buf.get_mut(..n), rest.get(..n)) else { return Ok(0) };
        into.copy_from_slice(from);
        self.at = self.at.saturating_add(n);
        Ok(n)
    }
}
