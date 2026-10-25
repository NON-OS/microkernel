// NONOS Operating System (AGPL-3.0-or-later)
//! _PLD: the revision, three bits of byte 8 and its panel field.

use super::super::gen::{lie, Rng};
use super::super::run::{elements, hold};
use crate::object::Object;
use crate::ports::{pld, Panel, Pld};

fn read(b: &[u8]) -> Option<Pld> {
    let revision = *b.first()? & 0x7F;
    let need = match revision {
        0 => return None,
        1 => 16,
        _ => 20,
    };
    if b.len() < need {
        return None;
    }
    let panel = [Panel::Top, Panel::Bottom, Panel::Left, Panel::Right, Panel::Front, Panel::Back]
        .get(usize::from(b[8] >> 3 & 7))
        .copied()
        .unwrap_or(Panel::Unknown);
    let bit = |n: u8| b[8] & 1 << n != 0;
    Some(Pld { revision, user_visible: bit(0), dock: bit(1), lid: bit(2), panel })
}

fn pld_model(o: &Object) -> Option<Pld> {
    match o {
        Object::Buffer(b) => read(&b.borrow()),
        _ => match elements(o)?.first()? {
            Object::Buffer(b) => read(&b.borrow()),
            _ => None,
        },
    }
}

fn place(r: &mut Rng) -> Object {
    let rev = [1, 2, 0, 0x81][r.below(4) as usize];
    let len = [16, 20, 12, 19][r.below(4) as usize];
    let mut b: Vec<u8> = (0..len).map(|_| r.next() as u8).collect();
    b[0] = rev;
    match r.below(3) {
        0 => Object::buffer(b),
        _ => lie(r, vec![Object::buffer(b)]),
    }
}

#[test]
fn pld_is_its_model() {
    hold("_PLD", 0x91D, place, pld, pld_model);
}
