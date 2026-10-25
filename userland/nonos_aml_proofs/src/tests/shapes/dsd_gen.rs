// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded _DSD packages: properties of every kind, nested past the bound,
//! references live and stale, and the set's UUID or partner lying.

use super::super::gen::{lie, stranger, Rng};
use crate::namespace::Id;
use crate::object::{Object, Ref};
use crate::props::DEVICE_PROPERTIES_UUID as UUID;

fn value(r: &mut Rng, clk: Id, live: u32, depth: u32) -> Object {
    match r.below(7) {
        0 => r.int(42),
        1 => Object::string(b"sony,imx\0tail"),
        2 => Object::Ref(Ref::Named(clk, live + r.below(2) as u32)),
        3 | 5 if depth < 6 => {
            let n = r.below(3);
            Object::package((0..n).map(|_| value(r, clk, live, depth + 1)).collect())
        }
        4 => Object::string(&[b'x'; 80]),
        _ => stranger(r),
    }
}

fn property(r: &mut Rng, clk: Id, live: u32) -> Object {
    let name = if r.below(8) == 0 { stranger(r) } else { Object::string(b"reset-gpios") };
    let v = value(r, clk, live, 0);
    lie(r, vec![name, v])
}

pub fn dsd(r: &mut Rng, clk: Id, live: u32) -> Object {
    let n = r.below(5);
    let props = (0..n).map(|_| property(r, clk, live)).collect();
    let set = lie(r, props);
    let mut uuid = UUID.to_vec();
    if r.below(6) == 0 {
        uuid[r.below(16) as usize] ^= 1;
    }
    let mut e = vec![Object::buffer(uuid), set];
    if r.below(4) == 0 {
        e.insert(0, Object::buffer(vec![0; 16]));
        e.insert(1, Object::package(vec![]));
    }
    // An odd slot holding the UUID is no key: the pair after it is read.
    if r.below(5) == 0 {
        e.insert(0, Object::Integer(0));
        e.insert(1, Object::buffer(UUID.to_vec()));
    }
    lie(r, e)
}
