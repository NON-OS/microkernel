// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in one batch's packet `i`, and its reply data: a packet
//! with part of a frame more, or past its share (both lost, counted); a
//! count past the 31 slots over bytes no slot names (only 31 read); a count short of the packets,
//! a length slot past the bytes, or a reply cut inside its slots (the rest
//! never seen).

use super::build::{reply, Rig, SLOTS};
use super::lies::Lie;
use super::rng::Rng;

pub struct Cost {
    pub lost: u32,
    pub unseen: usize,
}

pub fn told(lie: Lie, r: &mut Rng, rig: &mut Rig, ps: &mut [Vec<u8>], c: &mut Cost) -> Vec<u8> {
    let i = r.range(0, ps.len() - 1);
    let rest: usize = ps[i..].iter().map(Vec::len).sum();
    let mut count = ps.len() as u8;
    match lie {
        Lie::PartFrame => {
            let tail = r.range(1, rig.frame - 1);
            ps[i].extend(rig.bytes(tail));
            c.lost += tail as u32;
        }
        Lie::OverShare => {
            let k = r.range(1, 3 * rig.frame);
            ps[i] = rig.bytes(rig.share + k);
            c.lost += k as u32;
        }
        Lie::CountOver => {
            // Bytes after the packets, which no slot names: only a reader
            // that took slots past the 31st would find packets there.
            let mut d = reply(ps, r.range(SLOTS + 1, 255) as u8);
            d.extend((0..r.range(1, 3000)).map(|k| (k * 7) as u8));
            return d;
        }
        Lie::CountUnder => (count, c.unseen) = (i as u8, c.unseen + rest),
        Lie::LenPast => {
            let mut d = reply(ps, count);
            let past = (rest + r.range(1, 500)).min(u16::MAX as usize) as u16;
            d[4 + 2 * i..6 + 2 * i].copy_from_slice(&past.to_le_bytes());
            c.unseen += rest;
            return d;
        }
        Lie::ShortHeader => {
            let mut d = reply(ps, count);
            d.truncate(r.range(0, 4 + 2 * SLOTS - 1));
            c.unseen += ps.iter().map(Vec::len).sum::<usize>();
            return d;
        }
        _ => {}
    }
    reply(ps, count)
}
