// NONOS Operating System (AGPL-3.0-or-later)
//! A lie in one NDP of the chain, or in one of its entries.

use super::build::Block;
use super::lie::keep;
use super::lies::Lie;
use super::rng::Rng;

pub fn ndp_lie(r: &mut Rng, lie: Lie, b: &mut Block) -> Option<(bool, Vec<(usize, usize)>)> {
    let s = b.s;
    let j = r.range(0, b.ndps.len() - 1);
    let (at, grams) = b.ndps[j].clone();
    let expect = match lie {
        Lie::NdpLenShort => {
            b.put(at + 4, r.range(0, s.ndp_header + 2 * s.entry - 1) as u64, 2);
            keep(b, &|k, _| k < j)
        }
        Lie::NdpPastBlock => {
            b.put(at + 4, (b.raw.len() - at + r.range(1, 64)) as u64, 2);
            keep(b, &|k, _| k < j)
        }
        Lie::NdpSign => {
            b.raw[at + r.range(0, 3)] ^= 0x01;
            keep(b, &|k, _| k != j)
        }
        Lie::NdpLoop => {
            b.field(at + s.ndp_next, b.ndps[r.range(0, j)].0 as u64);
            keep(b, &|k, _| k <= j)
        }
        Lie::NextInHeader => {
            b.field(at + s.ndp_next, r.range(1, s.nth_len - 1) as u64);
            keep(b, &|k, _| k <= j)
        }
        Lie::NextPastBlock => {
            b.field(at + s.ndp_next, (b.raw.len() + r.range(0, 64)) as u64);
            keep(b, &|k, _| k <= j)
        }
        _ => {
            let i = r.range(0, grams.len() - 1);
            let (e, w) = (at + s.ndp_header + i * s.entry, s.entry / 2);
            match lie {
                Lie::ZeroIndex => b.field(e, 0),
                Lie::ShortDatagram => b.field(e + w, r.range(0, 13) as u64),
                Lie::DatagramPastBlock => {
                    b.field(e + w, (b.raw.len() - grams[i].0 + r.range(1, 64)) as u64)
                }
                Lie::DatagramWraps if s.wide => {
                    b.field(e, (u32::MAX - r.range(0, 32) as u32).into())
                }
                _ => return None,
            }
            keep(b, &|k, x| k != j || x < i)
        }
    };
    Some((true, expect))
}
