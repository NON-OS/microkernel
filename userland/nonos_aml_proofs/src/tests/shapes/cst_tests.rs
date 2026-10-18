// NONOS Operating System (AGPL-3.0-or-later)
//! _CST: a count, then per state a package of a Generic Register buffer
//! (ACPI 6.5, 6.4.3.7: tag 0x82, a 16-bit length of at least 12 that the
//! buffer holds, then space, width, offset, access and a 64-bit address),
//! the type 1 to 3, the latency and the power. Unlike _PSS, a bad state is
//! left out and the rest kept: idle states are chosen by latency. The
//! count is believed only as far as the package and a bound of 8 go.

use super::super::gen::{lie, Rng};
use super::super::run::{elements, hold, int, some, word};
use crate::cpu::{parse_cst, CState, Register};
use crate::object::Object;

fn register(o: &Object) -> Option<Register> {
    let Object::Buffer(b) = o else { return None };
    let b = b.borrow();
    let len = usize::from(u16::from_le_bytes([*b.get(1)?, *b.get(2)?]));
    if b[0] != 0x82 || len < 12 || b.len() < 3 + len {
        return None;
    }
    let address = u64::from_le_bytes(b[7..15].try_into().unwrap());
    Some(Register { space: b[3], width: b[4], offset: b[5], access: b[6], address })
}

fn state(o: &Object) -> Option<CState> {
    let e = elements(o)?;
    let kind = word(&e, 1).filter(|k| (1..=3).contains(k))? as u8;
    let (latency_us, milliwatts) = (word(&e, 2)?, word(&e, 3)?);
    Some(CState { register: register(e.first()?)?, kind, latency_us, milliwatts })
}

fn model(o: &Object) -> Option<Vec<CState>> {
    let e = elements(o)?;
    let n = usize::try_from(int(&e, 0)?).unwrap_or(usize::MAX).min(8);
    some(e.iter().skip(1).take(n).filter_map(state).collect())
}

fn reg(r: &mut Rng) -> Object {
    let mut b = vec![0x82, 12, 0, 1, 8, 0, 1];
    b.extend((r.next() & 0xFFFF).to_le_bytes());
    b.extend([0x79, 0]);
    match r.below(8) {
        0 => b[1] = r.below(40) as u8,
        1 => b.truncate(r.below(17) as usize),
        2 => b[0] = 0x86,
        _ => {}
    }
    Object::buffer(b)
}

fn cst(r: &mut Rng) -> Object {
    let n = r.below(5);
    let mut e = vec![Object::Integer([n, n + 3, 0, u64::MAX][r.below(4) as usize])];
    for _ in 0..n {
        let kind = [1, 2, 3, 1, 0, 4][r.below(6) as usize];
        let s = vec![reg(r), r.int(kind), r.int_below(500), r.int(1000)];
        e.push(lie(r, s));
    }
    lie(r, e)
}

#[test]
fn cst_is_its_model() {
    hold("_CST", 0xC57, cst, |o| some(parse_cst(o)), model);
}
