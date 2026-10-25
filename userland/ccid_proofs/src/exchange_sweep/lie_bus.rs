// NONOS Operating System (AGPL-3.0-or-later)
//! The lies the pipes and the controller tell, rather than the reader's
//! messages: a transfer failing or a command refused (a halt clears its
//! pipe), a command taken only in part, no answer at all, and more bytes
//! said to have come than were armed, which must be cut to `max`.

use super::build::Stream;
use super::lies::Lie;
use super::model::{E_IO, E_TIMEDOUT};
use super::rng::Rng;
use super::want::errno;

type Ends = (Result<Vec<u8>, i32>, Vec<bool>);

pub fn told_by_bus(lie: Lie, r: &mut Rng, s: &mut Stream, at: usize) -> Ends {
    match lie {
        Lie::Halt => {
            let (result, resets, e) = errno(r, true);
            s.transfers[at] = Err(e);
            (result, resets)
        }
        Lie::OutFails => {
            let (result, resets, e) = errno(r, false);
            s.out_fails = Some(e);
            (result, resets)
        }
        Lie::ShortWrite => {
            s.took = Some(r.range(0, s.cmd.len() - 1));
            (Err(E_IO), vec![])
        }
        Lie::Silence => {
            s.transfers.truncate(s.at);
            (Err(E_TIMEDOUT), vec![])
        }
        _ => {
            let mut m = s.answer.clone();
            let past = s.max + r.range(1, 64) - m.len();
            m.extend((0..past).map(|i| 0xD0 ^ i as u8));
            s.past = true;
            s.transfers[s.at] = Ok(Some(m.clone()));
            (Ok(m[..s.max].to_vec()), vec![])
        }
    }
}
