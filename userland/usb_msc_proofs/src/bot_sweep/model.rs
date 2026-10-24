// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound host makes of one BOT command, stated apart from the
//! driver (BOT 1.0, 5.3 and 6.7, as recalled, unverified). A CBW not taken
//! whole ends it. Data in is read a page at a time until a packet comes
//! short, empty or stalled. Thirteen bytes first, of a phase asked for more,
//! that carry "USBS" are a CSW sent in place of the data, none moved. Else
//! the CSW is read, again after one empty packet, again after one stall:
//! 13 bytes, signed, our tag, status 0 or 1, a residue within the length,
//! or else an I/O error and a reset, as when the device is silent. The
//! residue is the larger of the bytes that never came and the CSW's; once
//! an INQUIRY of 36 or READ CAPACITY(10) of 8 passes whole under a residue
//! all the same, the device's residues are never looked at again.

use super::build::{Ans, Cmd, Dir, PAGE};
use super::wire::take;

pub const E_IO: i32 = -5;

#[derive(Debug, PartialEq, Eq)]
pub struct Seen {
    pub result: Result<(u8, u32), i32>,
    pub used: usize,
}

pub fn rule(c: &Cmd, tag: u32, ignore: &mut bool) -> Seen {
    let (mut used, len) = (0, c.len);
    if c.cbw_takes.is_some_and(|k| k != 31) {
        return Seen { result: Err(E_IO), used };
    }
    let (mut moved, mut first) = (len * (c.dir != Dir::In) as usize, vec![]);
    while c.dir == Dir::In && moved < len {
        let n = PAGE.min(len - moved);
        let m = match take(&c.answers, &mut used) {
            None => return Seen { result: Err(E_IO), used: used.min(c.answers.len()) },
            Some(Ans::Data(b)) => b.len().min(n),
            Some(_) => 0,
        };
        if let (0, Some(Ans::Data(b))) = (moved, c.answers.get(used - 1)) {
            first = b[..m].to_vec();
        }
        moved += m;
        if m < n {
            break;
        }
    }
    let raw = if c.dir == Dir::In && moved == 13 && len > 13 && first.starts_with(b"USBS") {
        moved = 0;
        Some(first)
    } else {
        let mut got = take(&c.answers, &mut used);
        for again in [Ans::Zlp, Ans::Stall] {
            got = if got == Some(&again) { take(&c.answers, &mut used) } else { got };
        }
        if let Some(Ans::Data(b)) = got {
            Some(b[..b.len().min(13)].to_vec())
        } else {
            None
        }
    };
    let used = used.min(c.answers.len());
    let fail = Seen { result: Err(E_IO), used };
    let Some(w) = raw.filter(|w| w.len() == 13 && w.starts_with(b"USBS")) else { return fail };
    let word = |at: usize| u32::from_le_bytes(w[at..at + 4].try_into().unwrap());
    let (status, residue) = (w[12], if *ignore { 0 } else { word(8) });
    if word(4) != tag || status >= 2 || residue as usize > len {
        return fail;
    }
    let short = (len - moved) as u32;
    let probe = matches!((c.op, len), (0x12, 36) | (0x25, 8));
    if residue != 0 && probe && status == 0 && short == 0 {
        *ignore = true;
    }
    let residue = if *ignore || residue == 0 { short } else { short.max(residue) };
    Seen { result: Ok((status, residue)), used }
}
