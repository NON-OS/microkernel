// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::unwrap_used)]

use super::{read_whole, MAX_REPLY};
use crate::error::NetError;
use std::io::{Read, Result};
use std::time::Duration;

/// A peer that sends one byte at a time, each well inside a read's stall limit.
struct Trickle(usize);

impl Read for Trickle {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(2));
        let Some(first) = buf.first_mut().filter(|_| self.0 > 0) else { return Ok(0) };
        *first = b'x';
        self.0 -= 1;
        Ok(1)
    }
}

/// A peer that never stops sending.
struct Flood;

impl Read for Flood {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        buf.fill(b'x');
        Ok(buf.len())
    }
}

#[test]
fn a_trickling_peer_is_cut_off_at_the_whole_replys_time() {
    let mut slow = Trickle(usize::MAX);
    assert_eq!(read_whole(&mut slow, Duration::from_millis(30)), Err(NetError::TooSlow));
    let mut done = Trickle(5);
    assert_eq!(read_whole(&mut done, Duration::from_secs(5)).unwrap(), b"xxxxx");
}

#[test]
fn a_flood_past_the_cap_is_refused() {
    assert_eq!(read_whole(&mut Flood, Duration::from_secs(60)), Err(NetError::ReplyTooLarge));
    const { assert!(MAX_REPLY > 8192) };
}
