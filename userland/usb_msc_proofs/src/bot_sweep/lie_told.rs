// NONOS Operating System (AGPL-3.0-or-later)
//! Each lie told into its command, and the end it must come to.

use super::build::{Ans, Cmd, Dir, PAGE};
use super::lie::{status, wrap, Want};
use super::lie_more::more;
use super::lies::Lie;
use super::model::E_IO;
use super::rng::Rng;
use super::wire::{csw, pieces};

pub fn told(lie: Lie, r: &mut Rng, c: &mut Cmd, tag: u32, ignore: &mut bool) -> Want {
    let len = c.len as u32;
    match lie {
        Lie::ShortCbw | Lie::CswShaped | Lie::TrueResidue => return more(lie, r, c, tag),
        Lie::ShortUnsaid => {
            let d = r.range(0, c.len - 1);
            let st = status(c);
            (c.answers, c.delivered) = (pieces(&c.data, d), d);
            c.answers.push(Ans::Data(csw(b"USBS", tag, 0, st)));
            return Ok((st, len - d as u32));
        }
        Lie::DataStall => {
            let k = r.range(0, (c.len - 1) / PAGE);
            let st = status(c);
            (c.answers, c.delivered) = (pieces(&c.data[..k * PAGE], k * PAGE), k * PAGE);
            let res = [0, len - (k * PAGE) as u32][r.range(0, 1)];
            c.answers.extend([Ans::Stall, Ans::Data(csw(b"USBS", tag, res, st))]);
            return Ok((st, len - (k * PAGE) as u32));
        }
        Lie::EarlyCsw => {
            let st = status(c);
            c.answers = vec![Ans::Data(csw(b"USBS", tag, r.range(0, c.len) as u32, st))];
            c.delivered = 0;
            return Ok((st, len));
        }
        Lie::Bogus => {
            (c.answers, c.delivered) = (pieces(&c.data, c.len), c.len);
            c.answers.push(Ans::Data(csw(b"USBS", tag, r.range(1, c.len) as u32, 0)));
            *ignore = true;
            return Ok((0, 0));
        }
        Lie::ResiduePast => {
            let above = r.range(c.len + 2, u32::MAX as usize);
            wrap(c, b"USBS", tag, [len + 1, above as u32][r.range(0, 1)]);
        }
        Lie::Tag => wrap(c, b"USBS", [tag - 1, tag ^ (1 << r.range(0, 31))][r.range(0, 1)], 0),
        Lie::Signature => wrap(c, [b"USBC", b"SBSU", b"\0\0\0\0"][r.range(0, 2)], tag, 0),
        Lie::Status => {
            let st = r.range(2, 255) as u8;
            *c.answers.last_mut().unwrap() = Ans::Data(csw(b"USBS", tag, 0, st));
        }
        Lie::CswShort => {
            let Some(Ans::Data(w)) = c.answers.last_mut() else { unreachable!() };
            w.truncate(r.range(1, 12));
        }
        Lie::TwoEmpty | Lie::TwoStalls | Lie::Silent => {
            let w = c.answers.pop().unwrap();
            c.answers = if c.dir == Dir::In { pieces(&c.data, c.delivered) } else { vec![] };
            match lie {
                Lie::TwoEmpty => c.answers.extend([Ans::Zlp, Ans::Zlp, w]),
                Lie::TwoStalls => c.answers.extend([Ans::Stall, Ans::Stall, w]),
                _ => {}
            }
        }
    }
    Err(E_IO)
}
