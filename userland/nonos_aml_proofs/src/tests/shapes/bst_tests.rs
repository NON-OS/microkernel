// NONOS Operating System (AGPL-3.0-or-later)
//! _BST: the state word, then rate, remaining capacity and voltage.

use super::super::gen::{lie, Rng};
use super::super::run::{elements, hold};
use super::field;
use crate::object::Object;
use crate::power::{parse_bst, BatteryStatus};

fn model(o: &Object) -> Option<BatteryStatus> {
    let e = elements(o)?;
    let s = field(&e, 0)?;
    let lie = s > 0xF || s & 3 == 3 || e.len() < 4;
    (!lie).then(|| BatteryStatus {
        discharging: s & 1 != 0,
        charging: s & 2 != 0,
        critical: s & 4 != 0,
        rate: field(&e, 1),
        remaining: field(&e, 2),
        voltage: field(&e, 3),
    })
}

fn status(r: &mut Rng) -> Object {
    let state = [0, 1, 2, 4, 5, 6, 9, 3, 7, 0x10, 0x15][r.below(11) as usize];
    let e = vec![Object::Integer(state), r.int(1500), r.int(40000), r.int(12000)];
    lie(r, e)
}

#[test]
fn bst_is_its_model() {
    hold("_BST", 0xB57, status, parse_bst, model);
}
