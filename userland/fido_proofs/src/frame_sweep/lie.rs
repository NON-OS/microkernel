// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a sound stream, in message `j`, and the messages that
//! must still be heard whole: all but `j` when its reports lie; all but `j`
//! and the next when `j` never ends, since the next one opens on it; all of
//! them for a KEEPALIVE or ERROR heard as the key's.

use crate::ctaphid::frame::{packets, CMD_ERROR, PACKET};

use super::build::{keepalive, stream, Stream, CID};
pub use super::lies::{Lie, LIES};
use super::rng::Rng;

pub fn tell(r: &mut Rng) -> (Lie, Stream, Vec<usize>) {
    loop {
        let (lie, mut s) = (LIES[r.range(0, LIES.len() - 1)], stream(r));
        let j = r.range(0, s.messages.len() - 1);
        let (start, end) = (s.messages[j].2, s.messages[j].3);
        let conts = end - start - 1;
        let n = s.messages.len();
        let lost: Vec<usize> = match lie {
            Lie::SeqSkip if conts > 0 => {
                let at = r.range(start + 1, end - 1);
                s.reports[at][4] = (s.reports[at][4] + r.range(1, 126) as u8) & 0x7F;
                vec![j]
            }
            Lie::Reopen if conts > 0 => {
                let other = packets(CID, 0x90, &[9; 9]).unwrap()[0].to_vec();
                s.reports.insert(r.range(start + 1, end - 1), other);
                vec![j]
            }
            Lie::LengthPast => {
                // Half on the first length past the bound, the rest above it.
                let above = r.range(7611, 0xFFFF);
                let past = r.one_of(&[7610, above]) as u16;
                s.reports[start][5..7].copy_from_slice(&past.to_be_bytes());
                vec![j]
            }
            Lie::ShortReport | Lie::CutOff if conts > 0 => {
                let at = if lie == Lie::CutOff { end - 1 } else { r.range(start + 1, end - 1) };
                match lie {
                    Lie::CutOff => drop(s.reports.remove(at)),
                    _ => s.reports[at].truncate(r.range(0, 6)),
                }
                if at == end - 1 && j + 1 < n {
                    vec![j, j + 1]
                } else {
                    vec![j]
                }
            }
            Lie::BadKeepalive => {
                let mut k = keepalive(CID, r.one_of(&[0, 3, 0x7F, 0xFF]) as u8);
                if r.range(0, 1) == 0 {
                    (k[6], k[7]) = (r.range(2, 57) as u8, 1);
                }
                s.reports.insert(r.range(start, end), k);
                vec![]
            }
            Lie::ErrorOk | Lie::ErrorBadLength => {
                let mut e = vec![0u8; PACKET];
                e[..4].copy_from_slice(&CID);
                (e[4], e[6], e[7]) = (CMD_ERROR, 1, r.range(0, 255) as u8);
                let mid = conts > 0 && lie == Lie::ErrorBadLength;
                if lie == Lie::ErrorBadLength {
                    e[6] = r.one_of(&[0, 2, 9]) as u8;
                }
                s.reports.insert(if mid { r.range(start + 1, end - 1) } else { start }, e);
                mid.then_some(j).into_iter().collect()
            }
            _ => continue,
        };
        let kept = (0..n).filter(|m| !lost.contains(m)).collect();
        return (lie, s, kept);
    }
}
