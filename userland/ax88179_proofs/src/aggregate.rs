// NONOS Operating System (AGPL-3.0-or-later)
//! A receive transfer built as the chip builds one (Linux v6.12's comment
//! on its framing, read for behaviour): each frame behind 2 pad bytes and
//! padded to 8, then one table entry per frame, each followed by a filler
//! entry when asked, 4 more bytes when the trailer would not end on 8,
//! and the trailer: the entry count and the table's offset.

/// A frame and its entry's mark bits (bit 29 CRC error, bit 31 dropped).
pub type Frame = (Vec<u8>, u32);

pub const CRC_ERROR: u32 = 1 << 29;
pub const DROPPED: u32 = 1 << 31;

pub fn aggregate(frames: &[Frame], fillers: bool) -> Vec<u8> {
    let mut t = Vec::new();
    let mut table = Vec::new();
    for (f, marks) in frames {
        t.extend_from_slice(&[0xA5, 0x5A]);
        t.extend_from_slice(f);
        while t.len() % 8 != 0 {
            t.push(0xEE);
        }
        table.push((((f.len() + 2) as u32) << 16) | marks);
        if fillers {
            table.push(DROPPED);
        }
    }
    let at = t.len();
    for e in &table {
        t.extend_from_slice(&e.to_le_bytes());
    }
    if (t.len() + 4) % 8 != 0 {
        t.extend_from_slice(&[0; 4]);
    }
    let trailer = table.len() as u32 | (at as u32) << 16;
    t.extend_from_slice(&trailer.to_le_bytes());
    t
}

/// A frame of `len` bytes numbered `n`, so frames can be told apart.
pub fn frame(n: u8, len: usize) -> Vec<u8> {
    (0..len).map(|i| n.wrapping_mul(31).wrapping_add(i as u8)).collect()
}
