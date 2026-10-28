// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile transfers: random descriptors, honest packets among them, cut
//! anywhere. Every frame that comes up lies wholly inside the transfer, is
//! where the walk says, and the walk always ends.

use crate::rtl::rx::{next, Next, DESC, RX_BUFFER};
use crate::rx_tests::packet;

pub fn rand(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// A transfer of honest packets and lies, cut at a random length.
pub fn hostile(seed: &mut u64) -> Vec<u8> {
    let mut t = Vec::new();
    while t.len() < RX_BUFFER && !rand(seed).is_multiple_of(5) {
        t.resize(t.len().next_multiple_of(8), 0);
        match rand(seed) % 3 {
            0 => t.extend(packet(60 + (rand(seed) % 1460) as usize, 0, rand(seed) as u8)),
            1 => t.extend((0..rand(seed) % 64).map(|_| rand(seed) as u8)),
            // A descriptor alone, its length whatever the seed says.
            _ => t.extend(packet(0, rand(seed) as u32, 0x5a)),
        }
    }
    let cut = (rand(seed) % (t.len() as u64 + 1)) as usize;
    t.truncate(cut.min(RX_BUFFER - 1));
    t
}

/// Where the frames of `t` are: (start, len), each checked inside `t`.
pub fn walk(t: &[u8]) -> Vec<(usize, usize)> {
    let (mut at, mut out) = (0, Vec::new());
    for _ in 0..=t.len() {
        match next(t, at) {
            Next::Frame { start, len, next } => {
                assert!(start + len + 4 <= t.len() && start == at + DESC && next > at);
                out.push((start, len));
                at = next;
            }
            Next::End | Next::Broken => return out,
        }
    }
    panic!("the walk of a {}-byte transfer did not end", t.len());
}

#[test]
fn hostile_transfers_never_give_a_byte_from_outside_them() {
    let mut seed = 0x9e37_79b9_7f4a_7c15;
    for _ in 0..20_000 {
        let t = hostile(&mut seed);
        for (start, len) in walk(&t) {
            assert!((56..=32_763).contains(&len), "{len}");
            assert!(start + len <= t.len());
        }
    }
}
