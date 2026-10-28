// NONOS Operating System (AGPL-3.0-or-later)

//! Little-endian reads and writes into a region, and zeroing one.

use super::Region;

pub fn put16<R: Region + ?Sized>(r: &R, off: usize, v: u16) -> bool {
    r.write(off, &v.to_le_bytes())
}

pub fn put64<R: Region + ?Sized>(r: &R, off: usize, v: u64) -> bool {
    r.write(off, &v.to_le_bytes())
}

pub fn get16<R: Region + ?Sized>(r: &R, off: usize) -> Option<u16> {
    let mut b = [0u8; 2];
    r.read(off, &mut b).then(|| u16::from_le_bytes(b))
}

/// Zero `len` bytes at `off`, in chunks.
pub fn zero<R: Region + ?Sized>(r: &R, off: usize, len: usize) -> bool {
    let chunk = [0u8; 256];
    let mut done = 0;
    while done < len {
        let n = (len - done).min(chunk.len());
        let Some(at) = off.checked_add(done) else { return false };
        if !r.write(at, &chunk[..n]) {
            return false;
        }
        done += n;
    }
    true
}
