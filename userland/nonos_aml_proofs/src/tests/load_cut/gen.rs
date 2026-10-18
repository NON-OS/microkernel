// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded tables: two to five pieces, each a plain device, a device under
//! a load-time If, a Name given its value by a load-time Store, a device
//! with an EmbeddedControl region and a _REG, or a Name added into \_SB;
//! loaded directly, by LoadTable or by Load, after some steps of its own.

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Plain,
    Gated,
    Stored(u8),
    Ec,
    Into,
}

#[derive(Debug)]
pub struct Scene {
    pub pieces: Vec<Piece>,
    /// 0 directly, 1 by LoadTable, 2 by Load.
    pub path: u8,
    /// Steps \L takes before it loads.
    pub before: u8,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn scene(seed: u64) -> Scene {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let pieces = (0..2 + below(s, 4))
        .map(|_| match below(s, 5) {
            0 => Piece::Plain,
            1 => Piece::Gated,
            2 => Piece::Stored(1 + below(s, 200) as u8),
            3 => Piece::Ec,
            _ => Piece::Into,
        })
        .collect();
    Scene { pieces, path: below(s, 3) as u8, before: below(s, 4) as u8 }
}
