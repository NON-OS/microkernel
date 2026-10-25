// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in one command of a sound run of one to four, and what each
//! command must end in. A lie about the data phase leaves a status and the
//! bytes that never came; a lie about the CSW is an I/O error. After a
//! bogus residue on a probe, later residues are anything and count nothing.

use super::build::{sound, Ans, Cmd, Dir, PAGE};
use super::lie_told::told;
pub use super::lies::{Lie, LIES};
use super::rng::Rng;
use super::wire::csw;

pub type Want = Result<(u8, u32), i32>;

pub fn status(c: &Cmd) -> u8 {
    let Some(Ans::Data(w)) = c.answers.last() else { unreachable!() };
    w[12]
}

/// `c`'s CSW, its status kept, residue `res`; `sig` and `tag` as given.
pub fn wrap(c: &mut Cmd, sig: &[u8; 4], tag: u32, res: u32) {
    let st = status(c);
    *c.answers.last_mut().unwrap() = Ans::Data(csw(sig, tag, res, st));
}

pub fn tell(r: &mut Rng) -> (Lie, Vec<Cmd>, Vec<Want>) {
    let lie = LIES[r.range(0, LIES.len() - 1)];
    let (cmds, wants) = run(r, Some(lie));
    (lie, cmds, wants)
}

/// One to four commands, told `lie` in one of them.
pub fn run(r: &mut Rng, lie: Option<Lie>) -> (Vec<Cmd>, Vec<Want>) {
    let n = r.range(1, 4);
    let j = r.range(0, n - 1);
    let (mut cmds, mut wants, mut ignore) = (vec![], vec![], false);
    for i in 0..n {
        let tag = i as u32 + 1;
        let mut c = loop {
            let c = sound(r, tag);
            let fits = match lie {
                None => true,
                _ if i != j => true,
                Some(Lie::ShortUnsaid) => c.dir == Dir::In && c.len > 1,
                Some(Lie::DataStall) => c.dir == Dir::In && c.len > PAGE,
                Some(Lie::EarlyCsw) => c.dir == Dir::In && c.len > 13,
                Some(Lie::Bogus) => matches!((c.op, c.len), (0x12, 36) | (0x25, 8)),
                Some(Lie::CswShaped) => c.dir == Dir::In && c.len >= 13,
                Some(Lie::TrueResidue) => c.dir == Dir::In,
                _ => true,
            };
            if fits {
                break c;
            }
        };
        let short = (c.len - c.delivered) as u32;
        let mut want = Ok((status(&c), short));
        if c.dir != Dir::In && !ignore {
            let Some(Ans::Data(w)) = c.answers.last() else { unreachable!() };
            want = Ok((w[12], u32::from_le_bytes(w[8..12].try_into().unwrap())));
        }
        if ignore {
            wrap(&mut c, b"USBS", tag, r.range(0, u32::MAX as usize) as u32);
        }
        if let Some(lie) = lie.filter(|_| i == j) {
            want = told(lie, r, &mut c, tag, &mut ignore);
        }
        cmds.push(c);
        wants.push(want);
    }
    (cmds, wants)
}
