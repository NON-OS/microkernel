// NONOS Operating System (AGPL-3.0-or-later)
//! The stream as a blocking reader and writer, as the phones' stream reads and writes.

use super::{TorStream, STALL};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::time::Instant;

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
