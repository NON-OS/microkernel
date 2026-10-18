// NONOS Operating System (AGPL-3.0-or-later)
//! A CBW the device takes in part, an error and a reset; and two things a
//! device may do that leave the command whole: data that starts like a
//! CSW but is no early CSW (13 bytes asked, or more than 13 sent), and a
//! true residue on a phase that came whole, which a host cannot see and
//! takes (a probe's failed, so it gives no bogus residues away).

use super::build::{Ans, Cmd};
use super::lie::{status, Want};
use super::lies::Lie;
use super::model::E_IO;
use super::rng::Rng;
use super::wire::{csw, pieces};

pub fn more(lie: Lie, r: &mut Rng, c: &mut Cmd, tag: u32) -> Want {
    let (st, len) = (status(c), c.len);
    match lie {
        Lie::ShortCbw => {
            c.cbw_takes = Some(r.range(0, 30));
            return Err(E_IO);
        }
        Lie::CswShaped => {
            let w = csw(b"USBS", tag, r.range(0, len) as u32, r.range(0, 1) as u8);
            c.data[..13].copy_from_slice(&w);
            c.delivered = if len == 13 { 13 } else { r.range(14, len) };
        }
        _ => {
            let probe = matches!((c.op, len), (0x12, 36) | (0x25, 8));
            let st = if probe { 1 } else { st };
            let res = r.range(1, len) as u32;
            (c.answers, c.delivered) = (pieces(&c.data, len), len);
            c.answers.push(Ans::Data(csw(b"USBS", tag, res, st)));
            return Ok((st, res));
        }
    }
    let res = (len - c.delivered) as u32;
    c.answers = pieces(&c.data, c.delivered);
    c.answers.push(Ans::Data(csw(b"USBS", tag, res, st)));
    Ok((st, res))
}
