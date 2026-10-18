// NONOS Operating System (AGPL-3.0-or-later)
//! A sound bulk IN transfer of up to 4096 bytes: one to eight
//! REMOTE_NDIS_PACKET_MSGs back to back (Remote NDIS 1.0, 2.2.13, as
//! recalled, unverified), each a 44-byte header, sometimes a gap the
//! DataOffset steps over, an Ethernet frame of 14 to 1514 bytes and a few
//! bytes of padding inside MessageLength; zero padding after the last one
//! time in three.

pub use super::rng::Rng;

/// Each message's start and its frame, as offsets into the transfer.
pub struct Transfer {
    pub raw: Vec<u8>,
    pub msgs: Vec<(usize, core::ops::Range<usize>)>,
}

pub fn put(raw: &mut [u8], at: usize, v: u32) {
    raw[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

pub fn transfer(r: &mut Rng) -> Transfer {
    let budget = 64usize << r.range(0, 6);
    let mut t = Transfer { raw: Vec::new(), msgs: Vec::new() };
    for _ in 0..r.range(1, 8) {
        let (gap, frame, pad) = (r.one_of(&[0, 0, 4, 20]), r.range(14, 1514), r.range(0, 7));
        let len = 44 + gap + frame + pad;
        let full = t.raw.len() + len > 4096 || !t.msgs.is_empty() && t.raw.len() + len > budget;
        if full {
            break;
        }
        let at = t.raw.len();
        t.raw.extend((0..len).map(|i| (i as u8).wrapping_mul(29) | 1));
        t.raw[at..at + 44].fill(0);
        put(&mut t.raw, at, 1);
        put(&mut t.raw, at + 4, len as u32);
        put(&mut t.raw, at + 8, (36 + gap) as u32);
        put(&mut t.raw, at + 12, frame as u32);
        let start = at + 44 + gap;
        t.msgs.push((at, start..start + frame));
    }
    if r.range(0, 2) == 0 && t.raw.len() < 4096 {
        let pad = r.range(1, (4096 - t.raw.len()).min(64));
        t.raw.extend(core::iter::repeat_n(0, pad));
    }
    t
}
