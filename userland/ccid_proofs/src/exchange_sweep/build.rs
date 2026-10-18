// NONOS Operating System (AGPL-3.0-or-later)
//! A sound exchange (CCID 1.1, 5.2 and 6.2, as recalled, unverified): one
//! XfrBlock for slot 0 to 3 under any bSeq, then the reader's bulk IN
//! transfers, one message each. Idle looks and late answers (another slot
//! or sequence, any status) come between; up to EXTENSIONS of our own
//! time extensions, often all of them; then our answer, Done or Failed,
//! of 0 to the most data a message of `max` bytes holds.

use crate::ccid::{command, HEADER, XFR_BLOCK};
use crate::reader::answer;

use super::rng::Rng;

pub const DATA_BLOCK: u8 = 0x80;
/// The time extensions the driver takes before it gives up on a card.
pub const EXTENSIONS: usize = 64;

pub type Transfer = Result<Option<Vec<u8>>, i32>;

#[derive(Default)]
pub struct Stream {
    pub slot: u8,
    pub seq: u8,
    pub max: usize,
    pub cmd: Vec<u8>,
    pub out_fails: Option<i32>,
    /// What the lying bus says (bus.rs): command bytes taken, transfers whole.
    pub took: Option<usize>,
    pub past: bool,
    pub transfers: Vec<Transfer>,
    /// Where our answer stands in `transfers`, and the answer.
    pub at: usize,
    pub answer: Vec<u8>,
}

/// A message of `len` bytes of data from the reader.
pub fn message(slot: u8, seq: u8, status: u8, len: usize) -> Vec<u8> {
    answer(DATA_BLOCK, slot, seq, status, 0, &(0..len).map(|i| i as u8).collect::<Vec<_>>())
}

pub fn late(r: &mut Rng, s: &Stream) -> Vec<u8> {
    let (slot, seq) = match r.range(0, 1) {
        0 => ((s.slot + r.range(1, 3) as u8) & 3, s.seq),
        _ => (s.slot, s.seq.wrapping_add(r.range(1, 255) as u8)),
    };
    let len = r.range(0, s.max - HEADER);
    message(slot, seq, (r.range(0, 3) as u8) << 6 | r.range(0, 2) as u8, len)
}

fn filler(r: &mut Rng, s: &mut Stream) {
    for _ in 0..r.range(0, 2) {
        let t = if r.range(0, 1) == 0 { None } else { Some(late(r, s)) };
        s.transfers.push(Ok(t));
    }
}

pub fn stream(r: &mut Rng) -> Stream {
    let (slot, seq) = (r.range(0, 3) as u8, r.range(0, 255) as u8);
    let max = r.one_of(&[64, 271, 512, 4096]);
    let cmd = command(XFR_BLOCK, slot, seq, &[0x00, 0xA4, 0x04, 0x00, 0x00]);
    let mut s = Stream { slot, seq, max, cmd, ..Default::default() };
    let few = r.range(0, 4);
    for _ in 0..r.one_of(&[0, few, EXTENSIONS]) {
        filler(r, &mut s);
        s.transfers.push(Ok(Some(message(slot, seq, 0x80 | r.range(0, 2) as u8, 0))));
    }
    filler(r, &mut s);
    let some = r.range(0, max - HEADER);
    let len = r.one_of(&[0, max - HEADER, some]);
    s.answer = message(slot, seq, (r.range(0, 1) as u8) << 6 | r.range(0, 2) as u8, len);
    (s.at, s.answer[8]) = (s.transfers.len(), r.range(0, 255) as u8);
    s.transfers.push(Ok(Some(s.answer.clone())));
    s
}
