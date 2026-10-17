// NONOS Operating System (AGPL-3.0-or-later)
//! A function's AudioControl interface and its clock entities as bytes,
//! one of them cut short (its bLength with it) when a lie says, any other
//! bytes after; and the same entities, ids all 0, in a 1.0 control
//! interface and a streaming interface, before or after.

use super::build::{Ent, Func};
use crate::devices::{config, iface};
use crate::uac2::{clock_selector, clock_source, headphones_out, streaming_in};

pub fn bytes(e: &Ent) -> Vec<u8> {
    match e {
        Ent::Source { id, controls } => clock_source(*id, *controls),
        Ent::Selector { id, pins } => clock_selector(*id, pins),
        Ent::Terminal { id, input: true, clock } => streaming_in(*id, *clock),
        Ent::Terminal { id, clock, .. } => headphones_out(*id, 1, *clock),
    }
}

pub fn id_of(e: &mut Ent) -> &mut u8 {
    match e {
        Ent::Source { id, .. } | Ent::Selector { id, .. } | Ent::Terminal { id, .. } => id,
    }
}

/// The least an entity's descriptor holds.
pub fn least(e: &Ent) -> usize {
    match e {
        Ent::Source { .. } => 8,
        Ent::Selector { .. } => 6,
        Ent::Terminal { input: true, .. } => 17,
        Ent::Terminal { .. } => 12,
    }
}

pub fn encode(
    f: &Func,
    cut: Option<(usize, usize)>,
    extra: &[u8],
    foreign: Option<bool>,
) -> Vec<u8> {
    let mut parts = vec![iface(f.ac, 0, 1, 0x20)];
    for (i, e) in f.ents.iter().enumerate() {
        let mut b = bytes(e);
        if let Some((_, n)) = cut.filter(|c| c.0 == i) {
            b.truncate(n);
            b[0] = n as u8;
        }
        parts.push(b);
    }
    parts.push(extra.to_vec());
    if let Some(first) = foreign {
        let zero = |e: &Ent| {
            let mut e = e.clone();
            *id_of(&mut e) = 0;
            bytes(&e)
        };
        let junk: Vec<u8> = f.ents.iter().flat_map(zero).collect();
        let more = [iface(5, 0, 1, 0x00), junk.clone(), iface(6, 0, 2, 0x20), junk].concat();
        parts.insert(if first { 0 } else { parts.len() }, more);
    }
    let refs: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
    config(&refs)
}
