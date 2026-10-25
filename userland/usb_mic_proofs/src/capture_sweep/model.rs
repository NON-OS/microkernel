// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound host keeps of captured sound, stated apart from batch.rs
//! and ring.rs. A batch's packets are the first `count` (31 at most), each
//! its slot's length of the bytes after the slots, in order; one that runs
//! past the bytes ends the batch, and no later one is read. Of a packet,
//! the host keeps whole frames up to the packet's share; the rest is lost
//! and counted (USB Audio 2.0 2.3.1.1, as recalled, unverified). The ring
//! holds at most RING_CAP bytes of whole frames: to make room its oldest
//! frames go, and a packet larger than the ring keeps its newest; both
//! counted. Taking gives whole frames, oldest first, and the count lost
//! since the last take.

use std::collections::VecDeque;

use super::build::SLOTS;
use crate::ring::RING_CAP;

pub fn payloads(d: &[u8]) -> Vec<&[u8]> {
    let (mut out, mut at) = (vec![], 4 + 2 * SLOTS);
    if d.len() < at {
        return out;
    }
    for i in 0..(d[1] as usize).min(SLOTS) {
        let n = u16::from_le_bytes([d[4 + 2 * i], d[5 + 2 * i]]) as usize;
        let Some(p) = d.get(at..at + n) else { break };
        (out, at) = ([out, vec![p]].concat(), at + n);
    }
    out
}

pub struct Model {
    pub q: VecDeque<u8>,
    pub lost: u32,
    pub frame: usize,
    pub share: usize,
}

impl Model {
    pub fn packet(&mut self, p: &[u8]) {
        let whole = p.len().min(self.share) / self.frame * self.frame;
        self.lost += (p.len() - whole) as u32;
        let room = RING_CAP / self.frame * self.frame;
        let newest = &p[whole.saturating_sub(room)..whole];
        self.lost += (whole - newest.len()) as u32;
        while self.q.len() + newest.len() > RING_CAP {
            self.q.drain(..self.frame);
            self.lost += self.frame as u32;
        }
        self.q.extend(newest);
    }

    pub fn take(&mut self, most: usize) -> (u32, Vec<u8>) {
        let n = most.min(self.q.len()) / self.frame * self.frame;
        (std::mem::take(&mut self.lost), self.q.drain(..n).collect())
    }
}
