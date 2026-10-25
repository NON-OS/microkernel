// NONOS Operating System (AGPL-3.0-or-later)
//! A sound BOT command (BOT 1.0, 5 and 6.7, as recalled, unverified): an
//! INQUIRY, READ CAPACITY(10), REQUEST SENSE or READ(10) in, a WRITE(10)
//! out, or a command of no data. A device sends all it was asked for, or
//! ends short with a short or empty packet; a page at most a packet here.
//! Its CSW echoes the tag with the true residue and status 0 or 1, at times
//! after an empty packet, a stall, or both, in that order.

use super::rng::Rng;
use super::wire::{csw, pieces};

pub const PAGE: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ans {
    Data(Vec<u8>),
    Zlp,
    Stall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    In,
    Out,
    None,
}

pub struct Cmd {
    pub op: u8,
    pub dir: Dir,
    pub len: usize,
    /// What the device has to send, and how much of it it sends.
    pub data: Vec<u8>,
    pub delivered: usize,
    pub answers: Vec<Ans>,
    /// The bytes of the CBW the device says it took, when not all.
    pub cbw_takes: Option<usize>,
}

pub fn sound(r: &mut Rng, tag: u32) -> Cmd {
    let some = r.range(1, 3 * PAGE + 700);
    let (op, dir, len) = [
        (0x12, Dir::In, 36),
        (0x12, Dir::In, 96),
        (0x25, Dir::In, 8),
        (0x03, Dir::In, 18),
        (0x28, Dir::In, [13, 14, PAGE, PAGE + 1, 2 * PAGE, some][r.range(0, 5)]),
        (0x2A, Dir::Out, some),
        (0x00, Dir::None, 0),
    ][r.range(0, 6)];
    // Never 'U' first, so no data is taken for an early CSW.
    let data: Vec<u8> = (0..len).map(|i| (i * 7 + 1) as u8 | 0x80).collect();
    let short = r.range(0, len);
    let delivered = if dir == Dir::In { [len, len, short][r.range(0, 2)] } else { len };
    let residue = if dir == Dir::Out { r.range(0, len) } else { len - delivered };
    let mut answers = if dir == Dir::In { pieces(&data, delivered) } else { vec![] };
    let pre = [vec![], vec![Ans::Zlp], vec![Ans::Stall], vec![Ans::Zlp, Ans::Stall]];
    answers.extend(pre[[0, 0, 0, 1, 2, 3][r.range(0, 5)]].clone());
    answers.push(Ans::Data(csw(b"USBS", tag, residue as u32, r.range(0, 1) as u8)));
    Cmd { op, dir, len, data, delivered, answers, cbw_takes: None }
}
