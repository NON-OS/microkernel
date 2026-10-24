// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a sound exchange, and what it must end in: our answer
//! still, when the lie only passes something by; else the error it earns,
//! with the pipe cleared when it halted.

use crate::ccid::HEADER;

use super::build::{late, message, stream, Stream, EXTENSIONS};
use super::lie_bus::told_by_bus;
pub use super::lies::{Lie, LIES};
use super::model::{E_IO, E_TIMEDOUT};
use super::rng::Rng;
use super::want::Want;

pub fn tell(r: &mut Rng) -> (Lie, Stream, Want) {
    let (lie, mut s) = (LIES[r.range(0, LIES.len() - 1)], stream(r));
    let (at, ours) = (r.range(0, s.at), s.answer.clone());
    let mut result = Ok(ours.clone());
    let mut resets = vec![];
    let ask = (s.slot, s.seq);
    match lie {
        Lie::Cut => s.transfers.insert(at, Ok(Some(ours[..r.range(0, ours.len() - 1)].to_vec()))),
        Lie::TooLong => {
            let (slot, seq) = if r.range(0, 1) == 0 { ask } else { (s.slot ^ 1, s.seq) };
            let mut m = message(slot, seq, (r.range(0, 2) as u8) << 6, r.range(0, s.max - HEADER));
            let past = s.max - HEADER + 1;
            let above = past + r.range(1, 1 << 20);
            let len = r.one_of(&[past, above, u32::MAX as usize]);
            m[1..5].copy_from_slice(&(len as u32).to_le_bytes());
            s.transfers.insert(at, Ok(Some(m)));
            result = Err(E_IO);
        }
        Lie::BadStatus => {
            let m = message(s.slot, s.seq, 0xC0 | r.range(0, 2) as u8, r.range(0, s.max - HEADER));
            s.transfers.insert(at, Ok(Some(m)));
        }
        Lie::Overtime | Lie::LateFlood => {
            let more = |m: &Vec<u8>| m[5..7] == [ask.0, ask.1] && m[7] >> 6 == 2;
            let had = s.transfers.iter().filter(|t| matches!(t, Ok(Some(m)) if more(m))).count();
            let extra = [0, r.range(1, 8)][r.range(0, 1)];
            for _ in 0..EXTENSIONS + 1 - had + extra {
                let mut m = if lie == Lie::Overtime { s.answer.clone() } else { late(r, &s) };
                m[1..5].fill(0);
                m[7] = 0x80;
                m.truncate(HEADER);
                s.transfers.insert(s.at, Ok(Some(m)));
            }
            if lie == Lie::Overtime {
                result = Err(E_TIMEDOUT);
            }
        }
        Lie::ShortHeader => {
            let mut m = message(ask.0, ask.1, 0, 0);
            m[1..5].copy_from_slice(&(r.range(s.max - HEADER + 1, 1 << 24) as u32).to_le_bytes());
            m.truncate(r.range(5, HEADER - 1));
            s.transfers.insert(at, Ok(Some(m)));
        }
        Lie::Halt | Lie::OutFails | Lie::ShortWrite | Lie::Silence | Lie::PastBuffer => {
            (result, resets) = told_by_bus(lie, r, &mut s, at);
        }
        Lie::Trailing => {
            let mut m = ours;
            let room = s.max - m.len();
            m.extend((0..r.range(0, room)).map(|i| 0xE0 ^ i as u8));
            (s.transfers[s.at], result) = (Ok(Some(m.clone())), Ok(m));
        }
    }
    (lie, s, Want { result, resets })
}
