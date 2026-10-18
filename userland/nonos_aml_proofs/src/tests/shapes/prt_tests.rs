// NONOS Operating System (AGPL-3.0-or-later)
//! A _PRT entry under lying shapes. The model (ACPI 6.5, 6.2.13; recalled,
//! unverified): four elements, the address an integer within 32 bits whose
//! high word is the device, the pin an integer from 0 (INTA) to 3 (INTD),
//! the source Zero or a reference to a link device, and the index an
//! integer: with Zero it is the GSI, kept only when it fits 32 bits. Any
//! other shape is refused, so a lie never routes a pin to another line.

use super::gen::{lie, Rng};
use super::run::{elements, hold, int};
use crate::object::{Object, Ref};
use crate::platform::{prt_entry, PrtSource};

fn model(o: &Object) -> Option<(u16, u8, PrtSource)> {
    let e = elements(o)?;
    let address = u32::try_from(int(&e, 0)?).ok()?;
    let pin = int(&e, 1).filter(|p| *p <= 3)? as u8;
    let index = int(&e, 3)?;
    let source = match e.get(2)? {
        Object::Integer(0) => PrtSource::Gsi(u32::try_from(index).ok()),
        Object::Ref(Ref::Named(link, _)) | Object::Named(link, _) => PrtSource::Link(*link, index),
        _ => return None,
    };
    Some(((address >> 16) as u16, pin, source))
}

fn route(r: &mut Rng) -> Object {
    let device = r.below(32) << 16 | 0xFFFF;
    let pin = [0, 1, 2, 3, 4, 0x100][r.below(6) as usize];
    let source = match r.below(4) {
        0 => Object::Named(7, 0),
        1 => Object::Ref(Ref::Named(7, 0)),
        2 => Object::Integer(r.below(2) * 5),
        _ => Object::Integer(0),
    };
    let e = vec![r.int(device), r.int(pin), source, r.int_below(64)];
    lie(r, e)
}

#[test]
fn prt_entry_is_its_model() {
    let got = |o: &Object| prt_entry(&elements(o)?);
    hold("_PRT", 0x927, route, got, model);
}
