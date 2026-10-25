// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound host makes of one command's exchange, stated apart from
//! exchange.rs (CCID 1.1, 5.2 and 6.2, as recalled, unverified). A
//! command the pipe will not take, or takes in part, ends it, a halt
//! clearing the OUT pipe.
//! Then each look, while time is left, reads one transfer, cut to `max`:
//! an error ends it (a halt clears the IN pipe and reads as an I/O
//! error); nothing, or under the 10-byte header, is looked past. A
//! dwLength more than `max` can hold is an I/O error. A message cut short
//! of its dwLength, or of bStatus bits 7:6 = 3, is passed over, as is
//! anything not of our slot and bSeq. Ours is the answer unless it asks
//! for more time; each such restarts the wait, and the 65th gives up.

use super::build::{Stream, EXTENSIONS};

pub const E_IO: i32 = -5;
pub const E_PIPE: i32 = -32;
pub const E_TIMEDOUT: i32 = -110;

#[derive(Debug, PartialEq, Eq)]
pub struct Seen {
    pub result: Result<Vec<u8>, i32>,
    /// Waits begun afresh, and pipes cleared (IN true).
    pub fresh: u32,
    pub resets: Vec<bool>,
}

pub fn rule(s: &Stream, looks: usize) -> Seen {
    let (t, slot, seq, max) = (&s.transfers, s.slot, s.seq, s.max);
    let fail = |e: i32, dir_in: bool, fresh: u32| Seen {
        result: Err(if e == E_PIPE { E_IO } else { e }),
        fresh,
        resets: if e == E_PIPE { vec![dir_in] } else { vec![] },
    };
    if let Some(e) = s.out_fails {
        return fail(e, false, 0);
    }
    if s.took.is_some_and(|n| n != s.cmd.len()) {
        return fail(E_IO, false, 0);
    }
    let (mut fresh, mut extensions) = (1, 0);
    for look in 0.. {
        let done = |result, fresh| Seen { result, fresh, resets: vec![] };
        if look >= looks {
            return done(Err(E_TIMEDOUT), fresh);
        }
        let m = match t.get(look) {
            Some(Err(e)) => return fail(*e, true, fresh),
            Some(Ok(Some(m))) => &m[..m.len().min(max)],
            _ => continue,
        };
        let Some(h) = m.get(..10) else { continue };
        let len = u32::from_le_bytes([h[1], h[2], h[3], h[4]]) as usize;
        if len > max - 10 {
            return done(Err(E_IO), fresh);
        }
        if m.len() < 10 + len || h[7] >> 6 == 3 || h[5] != slot || h[6] != seq {
            continue;
        }
        if h[7] >> 6 != 2 {
            return done(Ok(m.to_vec()), fresh);
        }
        extensions += 1;
        if extensions > EXTENSIONS {
            return done(Err(E_TIMEDOUT), fresh);
        }
        fresh += 1;
    }
    unreachable!()
}
