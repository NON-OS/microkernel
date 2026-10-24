// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a function's clocks, and the clock the terminal asked
//! about must then run on: none when two entities share an id or one is
//! 0; as if absent an entity on its way cut short, or a selector of no
//! pins; none for a selector whose first pin is a selector, a clock
//! multiplier, or an id nothing has. Clock descriptors in a 1.0 control
//! interface or a streaming interface, whatever their ids, change nothing.

use super::build::{func, Ent, Func};
use super::encode::{encode, id_of, least};
pub use super::lies::{Lie, LIES};
use super::rng::Rng;
use super::want::{resolve, Seen};
use super::way::{point, way};

pub fn tell(r: &mut Rng) -> (Lie, Vec<u8>, u8, Option<Seen>) {
    let lie = LIES[r.range(0, LIES.len() - 1)];
    let mut f = func(r);
    let terms: Vec<u8> = f
        .ents
        .iter()
        .filter_map(|e| match e {
            Ent::Terminal { id, .. } => Some(*id),
            _ => None,
        })
        .collect();
    let t = terms[r.range(0, terms.len() - 1)];
    let (mut extra, mut cut, mut gone) = (vec![], None, None);
    let sel = f.ents.iter().position(|e| matches!(e, Ent::Selector { .. }));
    match (lie, sel) {
        (Lie::DupId | Lie::ZeroId, _) => {
            let (i, j) = (r.range(0, f.ents.len() - 1), r.range(0, f.ents.len() - 1));
            let to = if lie == Lie::ZeroId || i == j { 0 } else { *id_of(&mut f.ents[j]) };
            *id_of(&mut f.ents[i]) = to;
        }
        (Lie::Cut, _) => {
            let w = way(&f, t);
            let i = w[r.range(0, w.len() - 1)];
            (cut, gone) = (Some((i, r.range(2, least(&f.ents[i]) - 1))), Some(i));
        }
        (Lie::NoPins | Lie::Chain, Some(s)) => {
            let Ent::Selector { id, pins } = &mut f.ents[s] else { unreachable!() };
            let sid = *id;
            match lie {
                Lie::NoPins => (pins.clear(), gone = Some(s)).0,
                _ => pins[0] = 61,
            }
            if lie == Lie::Chain {
                f.ents.push(Ent::Selector { id: 61, pins: vec![62] });
            }
            point(&mut f, t, sid);
        }
        (Lie::NoPins | Lie::Chain, None) => return tell(r),
        (Lie::Multiplier | Lie::Missing, _) => {
            point(&mut f, t, 63);
            if lie == Lie::Multiplier {
                extra = vec![7, 0x24, 0x0C, 63, 1, 0, 0];
            }
        }
        _ => {}
    }
    let mut seen = Func { ac: f.ac, ents: f.ents.clone() };
    if let Some(g) = gone {
        seen.ents.remove(g);
    }
    let duped = matches!(lie, Lie::DupId | Lie::ZeroId);
    let want = if duped { None } else { resolve(&seen, t) };
    let foreign = (lie == Lie::Foreign).then(|| r.range(0, 1) == 0);
    (lie, encode(&f, cut, &extra, foreign), t, want)
}
