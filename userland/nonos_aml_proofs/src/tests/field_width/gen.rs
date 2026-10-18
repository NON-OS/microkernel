// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded field cases: the space, the region's base and length (just long
//! enough, or a little longer), the access type (AnyAcc to BufferAcc), the
//! update rule, the unit's offset and length (up to 100 bits, so some are
//! buffers), the bytes already there and the value to write.

use crate::enc::*;

pub struct Case {
    pub space: u8,
    pub base: u64,
    pub len: u64,
    pub access: u8,
    pub rule: u8,
    pub off: u32,
    pub bits: u32,
    /// The bytes from `base - 8` to `base + len + 8`.
    pub fill: Vec<u8>,
    pub value: u128,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn case(seed: u64) -> Case {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let space = [0, 1, 1, 0, 3][below(s, 5) as usize];
    let (access, rule) = (below(s, 6) as u8, below(s, 3) as u8);
    let most = if below(s, 4) == 0 { 100 } else { 64 };
    let bits = 1 + below(s, most) as u32;
    let off = below(s, 72) as u32;
    let need = (off + bits).div_ceil(8) as u64;
    let len = need + [0, 0, 1, 3, 8][below(s, 5) as usize];
    let step = [8, 1][below(s, 2) as usize];
    let base = [0x1000, 0x400, 0x20][space.min(2) as usize] + below(s, 16) * step;
    let fill = (0..len + 16).map(|_| below(s, 256) as u8).collect();
    let raw = u128::from(below(s, u64::MAX)) << 64 | u128::from(below(s, u64::MAX));
    let value = raw & ((1u128 << bits) - 1);
    Case { space, base, len, access, rule, off, bits, fill, value }
}

/// The region, the field, and \RD and \WR to read and write it.
pub fn source(c: &Case) -> Vec<u8> {
    let flags = c.access | c.rule << 5;
    let mut units = Vec::new();
    if c.off > 0 {
        units.push(skip(c.off as usize));
    }
    units.push(unit("FLD", c.bits as usize));
    cat(&[
        &region("RG", c.space, c.base, c.len),
        &field("RG", flags, &units),
        &method("\\RD", 0, &ret(&name("FLD"))),
        &method("\\WR", 1, &store(&arg(0), &name("FLD"))),
    ])
}
