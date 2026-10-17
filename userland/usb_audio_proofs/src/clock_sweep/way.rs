// NONOS Operating System (AGPL-3.0-or-later)
//! A terminal's way to its clock in a function as stated, and a terminal
//! pointed at another clock.

use super::build::{Ent, Func};
use super::encode::id_of;

pub fn find(f: &Func, id: u8) -> Option<usize> {
    f.ents.iter().position(|e| *id_of(&mut e.clone()) == id)
}

/// The terminal, then the entity it names, then a selector's first pin.
pub fn way(f: &Func, t: u8) -> Vec<usize> {
    let mut v = vec![find(f, t).unwrap()];
    while let Some(next) = match &f.ents[*v.last().unwrap()] {
        Ent::Terminal { clock, .. } => find(f, *clock),
        Ent::Selector { pins, .. } => find(f, pins[0]),
        Ent::Source { .. } => None,
    } {
        v.push(next);
    }
    v
}

pub fn point(f: &mut Func, t: u8, to: u8) {
    let i = find(f, t).unwrap();
    if let Ent::Terminal { clock, .. } = &mut f.ents[i] {
        *clock = to;
    }
}
