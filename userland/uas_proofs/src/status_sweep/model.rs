// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound host makes of one command's status pipe, stated apart from
//! nonos_uas (UAS 1.0, as recalled, unverified): with streams the data
//! moves first; then each IU read. An IU it cannot read (under 4 bytes, a
//! Sense IU under 16 or whose sense runs past it, a Response IU under 8, an
//! id of no status kind) is a transport error. One under another tag is
//! let go, and the ninth such ends the command refused (02h). Under the
//! command's tag: READ or WRITE READY, for a USB 2 device that has moved
//! nothing yet the way the command moves, starts the data, any other is
//! refused (02h); a Sense IU ends the command with its status and sense; a
//! Response IU refuses it with its code.

use nonos_uas::DataDir;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Done(u8, Vec<u8>),
    Failed(u8),
    Unreadable,
    /// The pipe ran dry: the generator never lets it.
    Starved,
}

pub fn outcome(tag: u16, dir: DataDir, streams: bool, ius: &[Vec<u8>]) -> Outcome {
    let mut next = ius.iter();
    let (mut moved, mut dropped) = (streams && dir != DataDir::None, 0);
    loop {
        let Some(b) = next.next() else { return Outcome::Starved };
        if b.len() < 4 {
            return Outcome::Unreadable;
        }
        let t = u16::from_be_bytes([b[2], b[3]]);
        let ready = match b[0] {
            0x06 => Some(DataDir::In),
            0x07 => Some(DataDir::Out),
            0x03 if b.len() >= 16
                && 16 + u16::from_be_bytes([b[14], b[15]]) as usize <= b.len() =>
            {
                None
            }
            0x04 if b.len() >= 8 => None,
            _ => return Outcome::Unreadable,
        };
        if t != tag {
            dropped += 1;
            if dropped > 8 {
                return Outcome::Failed(0x02);
            }
            continue;
        }
        match (b[0], ready) {
            (_, Some(d)) if !streams && !moved && d == dir => moved = true,
            (_, Some(_)) => return Outcome::Failed(0x02),
            (0x03, _) => {
                let n = u16::from_be_bytes([b[14], b[15]]) as usize;
                return Outcome::Done(b[6], b[16..16 + n].to_vec());
            }
            _ => return Outcome::Failed(b[7]),
        }
    }
}
