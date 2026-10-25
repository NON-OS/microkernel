// NONOS Operating System (AGPL-3.0-or-later)
//! The flush's lies: done when the device flushes, or fails it with a
//! sense of ILLEGAL REQUEST (no cache it lets a host flush); an I/O error
//! when the sense says otherwise, came short, is not fixed format, or
//! REQUEST SENSE failed too.

use super::build::Reply;
use super::lie::{Lie, Want};
use super::model::E_IO;
use crate::bot_sweep::rng::Rng;

pub fn flush(lie: Lie, r: &mut Rng, mut s: Reply, n: usize) -> (Reply, Reply, Want) {
    let mut cmd = Reply::whole(vec![]);
    cmd.status = (lie != Lie::FlushOk) as u8;
    let done = matches!(lie, Lie::FlushOk | Lie::FlushIllegal);
    let want = if done { Ok(None) } else { Err(E_IO) };
    // The sense lies say ILLEGAL REQUEST, so only the lie keeps the flush
    // from being taken as done.
    if matches!(lie, Lie::SenseShort | Lie::SenseFormat | Lie::SenseFails) {
        s.data[2] = 0x05;
    }
    match lie {
        Lie::FlushIllegal => s.data[2] = 0x05 | (n as u8 & 0xF0),
        Lie::SenseShort => {
            s.sent = r.range(0, 18);
            s.residue = (18 - r.range(0, 13)).max(18 - s.sent) as u32;
        }
        Lie::SenseFormat => s.data[0] = [0x72, 0x73, 0x00, 0x7F][r.range(0, 3)] | (n as u8 & 0x80),
        Lie::SenseFails => s.status = 1,
        _ => {}
    }
    (cmd, s, want)
}
