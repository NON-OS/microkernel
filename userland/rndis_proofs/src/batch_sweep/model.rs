// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound reader takes from a bulk IN transfer, stated apart from the
//! driver (Remote NDIS 1.0, 2.2.13, as recalled, unverified): messages back
//! to back while each is a PACKET_MSG whose MessageLength holds its 44-byte
//! header and stays in the transfer; of each, its frame when DataOffset
//! (counted from byte 8) is past the header and the frame, 14 to 1514
//! bytes, lies inside the message. A message whose frame fails is dropped
//! and the walk goes on; one of another type or a bad length ends it, and
//! is dropped unless it is all-zero padding.

use core::ops::Range;

fn le(b: &[u8], at: usize) -> Option<u64> {
    Some(u32::from_le_bytes(b.get(at..at.checked_add(4)?)?.try_into().ok()?) as u64)
}

pub fn frames(t: &[u8]) -> (Vec<Range<usize>>, u32) {
    let (mut at, mut out, mut dropped) = (0usize, Vec::new(), 0u32);
    while let (Some(kind), Some(len)) = (le(t, at), le(t, at + 4)) {
        if kind != 1 || len < 44 || at as u64 + len > t.len() as u64 {
            dropped += (kind != 0 || len != 0) as u32;
            break;
        }
        let (off, flen) = (le(t, at + 8).unwrap(), le(t, at + 12).unwrap());
        let start = at as u64 + 8 + off;
        let fits = off >= 36 && (14..=1514).contains(&flen) && start + flen <= at as u64 + len;
        match fits {
            true => out.push(start as usize..(start + flen) as usize),
            false => dropped += 1,
        }
        at += len as usize;
    }
    (out, dropped)
}
