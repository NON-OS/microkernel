// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in an honest device's answers to sizing or a flush, and
//! the end it must come to: a size, a flush done, or an I/O error.

use super::build::{cap10, cap16, lba32, sense, served, Reply};
use super::lie_flush::flush;
pub use super::lies::{Lie, LIES};
use super::model::E_IO;
use crate::bot_sweep::rng::Rng;

pub enum Case {
    Size(Reply, Reply, Reply),
    Flush(Reply, Reply),
}

pub type Want = Result<Option<(u64, u32)>, i32>;

pub fn tell(r: &mut Rng) -> (Lie, Case, Want) {
    let lie = LIES[r.range(0, LIES.len() - 1)];
    let (lba, bl, last) = (lba32(r), served(r), r.range(0, usize::MAX - 1) as u64);
    let (mut c10, mut c16) = (cap10(lba, bl), cap16(last, served(r)));
    let code = [0x70, 0x71, 0xF0, 0xF1][r.range(0, 3)];
    let s = sense(code, [0x02, 0x03, 0x06][r.range(0, 2)], r.range(0, 255) as u8);
    let mut want = Ok(Some((lba as u64 + 1, bl)));
    if (Lie::Big as usize..=Lie::Residue16 as usize).contains(&(lie as usize)) {
        c10 = cap10(u32::MAX, bl);
        want = Ok(Some((last + 1, c16.data[8..12].iter().fold(0, |a, b| a << 8 | *b as u32))));
    }
    let n = r.range(0, 255);
    match lie {
        Lie::Short10 | Lie::Short16Bad => {
            let (c, most) = if lie == Lie::Short10 { (&mut c10, 7) } else { (&mut c16, 11) };
            c.sent = r.range(0, most);
            c.residue = [0, (c.data.len() - c.sent) as u32][r.range(0, 1)];
            want = Err(E_IO);
        }
        Lie::Bogus10 => c10.residue = r.range(1, 8) as u32,
        Lie::Past10 => (c10.residue, want) = ([9, n as u32 + 9][r.range(0, 1)], Err(E_IO)),
        Lie::Fail10 | Lie::Fail16 => {
            let c = if lie == Lie::Fail10 { &mut c10 } else { &mut c16 };
            (c.status, want) = (1, Err(E_IO));
        }
        Lie::Short16Ok => (c16.sent, c16.residue) = (n % 20 + 12, 20 - (n % 20) as u32),
        Lie::Last16Max => (c16, want) = (cap16(u64::MAX, 512), Err(E_IO)),
        Lie::Residue16 => {
            c16.residue = r.range(1, 32) as u32;
            want = want.and_then(|w| if c16.residue <= 20 { Ok(w) } else { Err(E_IO) });
        }
        Lie::OddLen => {
            let odd = [0, 1, 513, 8192, u32::MAX][r.range(0, 4)];
            (c10, want) = (cap10(lba, odd), Ok(Some((lba as u64 + 1, odd))));
        }
        _ => {}
    }
    if (lie as usize) < Lie::FlushOk as usize {
        return (lie, Case::Size(c10, c16, s), want);
    }
    let (cmd, s, want) = flush(lie, r, s, n);
    (lie, Case::Flush(cmd, s), want)
}
