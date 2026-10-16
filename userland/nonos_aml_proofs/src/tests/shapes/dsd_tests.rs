// NONOS Operating System (AGPL-3.0-or-later)
//! _DSD under lying shapes. The model (the _DSD device properties
//! documents; recalled, unverified): the elements pair a UUID buffer with
//! a package; the first pair whose buffer is the UUID asked for is read,
//! and when its partner is no package nothing is. Each property is a
//! package of a String name and a value: an integer, a string to its NUL,
//! a reference to an object still alive, or a package of such values four
//! deep at most. Anything else is dropped, never read as another type.
//! Names, strings, packages and sets keep 64 entries at most.

use super::gen::Rng;
use super::run::{elements, hold, some};
use crate::enc::device;
use crate::load::Aml;
use crate::object::{Object, Ref};
use crate::props::{properties_in, Property, Value, DEVICE_PROPERTIES_UUID as UUID};
use crate::run::boot;

fn text(s: &[u8]) -> Vec<u8> {
    s.iter().copied().take_while(|c| *c != 0).take(64).collect()
}

fn value(aml: &Aml, o: &Object, depth: u32) -> Option<Value> {
    match o {
        Object::Integer(v) => Some(Value::Integer(*v)),
        Object::String(s) => Some(Value::String(text(&s.borrow()))),
        Object::Ref(Ref::Named(id, g)) if aml.ns.live(*id, *g) => Some(Value::Reference(*id)),
        Object::Package(p) if depth < 4 => {
            let p = p.borrow();
            Some(Value::Package(
                p.iter().take(64).filter_map(|e| value(aml, e, depth + 1)).collect(),
            ))
        }
        _ => None,
    }
}

fn model(aml: &Aml, o: &Object) -> Vec<Property> {
    let e = elements(o).unwrap_or_default();
    let ours = |b: &Object| matches!(b, Object::Buffer(b) if *b.borrow() == UUID);
    let Some(i) = (0..e.len()).step_by(2).find(|i| ours(&e[*i])) else { return Vec::new() };
    let Some(set) = e.get(i + 1).and_then(|s| matches!(s, Object::Package(_)).then_some(s)) else {
        return Vec::new();
    };
    let prop = |p: &Object| {
        let p = elements(p)?;
        let Object::String(name) = p.first()? else { return None };
        let name = text(&name.borrow());
        Some(Property { name, value: value(aml, p.get(1)?, 0)? })
    };
    elements(set).unwrap().iter().take(64).filter_map(prop).collect()
}

#[path = "dsd_gen.rs"]
mod made;

#[test]
fn properties_in_is_its_model() {
    let (aml, _) = boot(&device("\\_SB.CLK0", &[]), 2);
    let clk = crate::consts::at(&aml, "\\_SB.CLK0").unwrap();
    let live = aml.ns.get(clk).unwrap().gen;
    let got = |o: &Object| some(properties_in(&aml, &elements(o).unwrap_or_default(), &UUID));
    let make = |r: &mut Rng| made::dsd(r, clk, live);
    hold("_DSD", 0xD5D, make, got, |o| some(model(&aml, o)));
}
