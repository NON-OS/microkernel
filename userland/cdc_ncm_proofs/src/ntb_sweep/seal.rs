// NONOS Operating System (AGPL-3.0-or-later)
//! Writing a sound NTB's header and NDPs over its bytes.

use super::build::Block;
use super::rng::Rng;

/// Write the header and every NDP as `b.ndps` lists them, the block
/// `end` bytes long (its length field 0 one time in four).
pub fn seal(r: &mut Rng, b: &mut Block, end: usize) {
    let s = b.s;
    b.raw.truncate(end.max(s.nth_len));
    for (i, x) in b.raw.iter_mut().enumerate() {
        *x = (i as u8).wrapping_mul(31);
    }
    b.put(0, s.nth_sign.into(), 4);
    b.put(4, s.nth_len as u64, 2);
    b.field(8, if r.range(0, 3) == 0 { 0 } else { end as u64 });
    let ndps = b.ndps.clone();
    b.field(if s.wide { 12 } else { 10 }, ndps[0].0 as u64);
    for (k, (at, grams)) in ndps.iter().enumerate() {
        let next = ndps.get(k + 1).map_or(0, |n| n.0) as u64;
        b.put(*at, s.ndp_sign.into(), 4);
        b.put(at + 4, (s.ndp_header + (grams.len() + 1) * s.entry) as u64, 2);
        b.field(at + s.ndp_next, next);
        for (j, &(index, len)) in grams.iter().chain([(0, 0)].iter()).enumerate() {
            let e = at + s.ndp_header + j * s.entry;
            b.field(e, index as u64);
            b.field(e + s.entry / 2, len as u64);
        }
    }
    b.rx_max = r.range(b.raw.len().max(2048), 4096);
}
