// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a sound configuration, and what the walk owes after it.

use super::build::cs_least;
use super::frame_lie::{overlong, past_end, pick_where};
use super::model::least;
use super::rng::Rng;

pub use super::lies::{Lie, LIES};
use Lie::*;

/// The configuration as told, the descriptors from its start the walk
/// must give as built, and whether it must stop there (false: what
/// follows the lie is the device's to frame, and only safety is owed).
pub struct Told {
    pub lie: Lie,
    pub descs: Vec<Vec<u8>>,
    pub framed: usize,
    pub ends: bool,
}

pub fn tell(r: &mut Rng, built: &[Vec<u8>]) -> Told {
    loop {
        let (lie, mut descs) = (r.pick(&LIES), built.to_vec());
        if let Some((framed, ends)) = apply(r, lie, &mut descs) {
            return Told { lie, descs, framed, ends };
        }
    }
}

fn apply(r: &mut Rng, lie: Lie, d: &mut Vec<Vec<u8>>) -> Option<(usize, bool)> {
    let n = d.len();
    match lie {
        ZeroLen | OneLen => {
            let i = r.range(1, n - 1);
            d[i][0] = (lie == OneLen) as u8;
            return Some((i, true));
        }
        PastEnd => return past_end(r, d),
        Overlong => return overlong(r, d),
        ShortForType => {
            let i = pick_where(r, d, |x| least(x) > 2)?;
            let len = r.range(2, least(&d[i]) - 1);
            d[i].truncate(len);
            d[i][0] = len as u8;
        }
        ShortTwin => {
            let i = pick_where(r, d, |x| x[1] == 0x24 && cs_least(x[2]) > 3)?;
            let len = r.range(3, cs_least(d[i][2]) - 1);
            let twin = [&[len as u8], &d[i][1..len]].concat();
            d.insert(i, twin);
        }
        _ => super::field_lie::field(r, lie, d)?,
    }
    Some((d.len(), true))
}
