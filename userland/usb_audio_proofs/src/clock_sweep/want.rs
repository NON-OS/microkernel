// NONOS Operating System (AGPL-3.0-or-later)
//! The clock a terminal of a function as stated runs on (build.rs).

use super::build::{Ent, Func};

/// A clock as compared: its AudioControl interface, its id, whether the
/// host sets its rate, and the selector before it.
pub type Seen = (u8, u8, bool, Option<u8>);

/// The clock terminal `t` runs on, as stated.
pub fn resolve(f: &Func, t: u8) -> Option<Seen> {
    let source = |id: u8| {
        f.ents.iter().find_map(|e| match e {
            Ent::Source { id: s, controls } if *s == id => Some(controls & 3 == 3),
            _ => None,
        })
    };
    let at = f.ents.iter().find_map(|e| match e {
        Ent::Terminal { id, clock, .. } if *id == t => Some(*clock),
        _ => None,
    })?;
    let (selector, id) = match source(at) {
        Some(_) => (None, at),
        None => f.ents.iter().find_map(|e| match e {
            Ent::Selector { id, pins } if *id == at => Some((Some(*id), pins[0])),
            _ => None,
        })?,
    };
    Some((f.ac, id, source(id)?, selector))
}
