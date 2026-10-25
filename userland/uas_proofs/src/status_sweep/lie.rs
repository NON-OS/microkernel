// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told on a command's status pipe, and what the command must come
//! to: foreign IUs let go up to eight and the ninth refuses it; an IU that
//! cannot be read is a transport error; a Response IU refuses it with its
//! code; a READY out of turn refuses it; a Sense IU with no READY before it
//! still ends it.

use super::build::{exchange, iu, Exchange, READ_READY, RESPONSE, SENSE, WRITE_READY};
pub use super::lies::{Lie, LIES};
use super::model::Outcome;
use super::ready::return_ready;
use super::rng::Rng;

pub fn tell(r: &mut Rng) -> (Lie, Exchange, Outcome) {
    loop {
        let (lie, mut x) = (LIES[r.range(0, LIES.len() - 1)], exchange(r));
        let done = Outcome::Done(x.status, x.sense.clone());
        let last = x.ius.len() - 1;
        let want = match lie {
            Lie::Foreign => {
                let k = r.range(1, 12);
                for _ in 0..k {
                    let other = x.tag.wrapping_add(r.range(1, 0xFFFE) as u16).max(1);
                    let id =
                        r.one_of(&[READ_READY as usize, WRITE_READY as usize, RESPONSE as usize])
                            as u8;
                    let at = r.range(0, x.ius.len() - 1);
                    x.ius.insert(at, iu(id, other, 8));
                }
                if k <= 8 {
                    done
                } else {
                    Outcome::Failed(0x02)
                }
            }
            Lie::SensePast => {
                let past = (x.ius[last].len() - 16 + r.range(1, 64)) as u16;
                x.ius[last][14..16].copy_from_slice(&past.to_be_bytes());
                Outcome::Unreadable
            }
            Lie::Short => {
                let at = r.range(0, last);
                let cut = if x.ius[at][0] == SENSE { r.range(0, 15) } else { r.range(0, 3) };
                x.ius[at].truncate(cut);
                Outcome::Unreadable
            }
            Lie::Unknown => {
                let at = r.range(0, last);
                x.ius[at][0] = r.one_of(&[0, 1, 2, 5, 8, 0x55, 0xFF]) as u8;
                Outcome::Unreadable
            }
            Lie::Response => {
                let code = r.range(0, 255) as u8;
                x.ius[last] = iu(RESPONSE, x.tag, 8);
                x.ius[last][7] = code;
                Outcome::Failed(code)
            }
            Lie::ShortResponse => {
                x.ius[last] = iu(RESPONSE, x.tag, r.range(4, 7));
                Outcome::Unreadable
            }
            Lie::WrongReady => return_ready(r, &mut x),
            Lie::SenseFirst if last == 1 => (x.ius.remove(0), done).1,
            Lie::SenseFirst => continue,
        };
        return (lie, x, want);
    }
}
