// NONOS Operating System (AGPL-3.0-or-later)
//! Sound capture (USB Audio 2.0 2.3.1.1, as recalled, unverified): frames
//! of 2 bytes a channel, one to eight channels; each service interval's
//! packet carries its share of frames, a frame more or less as the rates
//! drift, never more than the share and one frame. The controller's batch
//! reply holds a completion code, the packet count, 31 u16 length slots,
//! then the bytes; at most BATCH_BYTES of them. Every byte is numbered so
//! a byte out of order shows.

use super::rng::Rng;

pub const SLOTS: usize = 31;
pub const BATCH_BYTES: usize = 4034;

pub struct Rig {
    pub frame: usize,
    /// Frames a packet carries at the nominal rate.
    pub per: usize,
    /// The most a packet may carry: `per` frames and one more.
    pub share: usize,
    pub next: u8,
}

impl Rig {
    pub fn new(r: &mut Rng) -> Rig {
        let frame = 2 * r.range(1, 8);
        let per = r.range(1, 48);
        Rig { frame, per, share: (per + 1) * frame, next: 0 }
    }

    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n)
            .map(|_| {
                self.next = self.next.wrapping_add(1);
                self.next
            })
            .collect()
    }

    /// One sound packet: `per` frames, one fewer or one more, or none.
    pub fn packet(&mut self, r: &mut Rng) -> Vec<u8> {
        let frames = [self.per, self.per - 1, self.per + 1, 0][r.range(0, 3)];
        self.bytes(frames * self.frame)
    }

    /// A sound batch: as many packets as fit, one to 31.
    pub fn batch(&mut self, r: &mut Rng) -> Vec<Vec<u8>> {
        let most = (BATCH_BYTES / self.share).clamp(1, SLOTS);
        (0..r.range(1, most)).map(|_| self.packet(r)).collect()
    }
}

/// The reply data carrying `packets`, saying `count` of them.
pub fn reply(packets: &[Vec<u8>], count: u8) -> Vec<u8> {
    let mut d = vec![0, count, 0, 0];
    for i in 0..SLOTS {
        let n = packets.get(i).map_or(0, |p| p.len() as u16);
        d.extend(n.to_le_bytes());
    }
    d.extend(packets.concat());
    d
}
