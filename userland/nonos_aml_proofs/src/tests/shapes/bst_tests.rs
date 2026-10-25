// NONOS Operating System (AGPL-3.0-or-later)
//! _BST: the state word, then rate, remaining capacity and voltage. The
//! the maintainers' rule: state bits past bit 3 are ignored and kept to be
//! said; charging and discharging at once leave the direction unknown,
//! the rest kept; and the critical bit is kept in both cases, so it still
//! reaches the power-off. Only a state word that is no known integer, or a
//! package too short, is refused.

use super::super::gen::{lie, Rng};
use super::super::run::{elements, hold};
use super::field;
use crate::object::Object;
use crate::power::{parse_bst, BatteryStatus};

fn model(o: &Object) -> Option<BatteryStatus> {
    let e = elements(o)?;
    let s = field(&e, 0)?;
    let both = s & 3 == 3;
    (e.len() >= 4).then(|| BatteryStatus {
        discharging: s & 1 == 1 && !both,
        charging: s & 2 == 2 && !both,
        contradicts: both,
        critical: s & 4 == 4,
        reserved: s >> 4 << 4,
        rate: field(&e, 1),
        remaining: field(&e, 2),
        voltage: field(&e, 3),
    })
}

fn status(r: &mut Rng) -> Object {
    let state = [0, 1, 2, 4, 5, 6, 9, 3, 7, 0x10, 0x15, 0x8000_0007][r.below(12) as usize];
    let e = vec![Object::Integer(state), r.int(1500), r.int(40000), r.int(12000)];
    lie(r, e)
}

#[test]
fn bst_is_its_model() {
    hold("_BST", 0xB57, status, parse_bst, model);
}

/// What reached the power-off before F-106 (the discharging and critical
/// bits) still does, whatever else the word says; and a direction is
/// never claimed when firmware gave both. (All 32 bits set is the
/// unknown value, refused as any unknown field is: no state at all.)
#[test]
fn the_critical_bit_survives_every_lie() {
    let high = [0, 0x10, 0x80, 0x100, 0x8000_0000, 0xFFFF_FFE0];
    for state in (0..16u64).flat_map(|low| high.map(|h| h | low)) {
        let e = vec![Object::Integer(state), Object::Integer(10), Object::Integer(5)];
        let s = parse_bst(&Object::package([e, vec![Object::Integer(0)]].concat()));
        let s = s.unwrap_or_else(|| panic!("state {state:#x} refused"));
        let was = state & 5 == 5;
        assert!(!was || (s.may_discharge() && s.critical), "state {state:#x} lost its critical");
        assert!(state & 3 != 3 || !(s.charging || s.discharging), "state {state:#x}");
        assert!(!s.may_discharge() || state & 1 == 1, "state {state:#x} invented discharging");
        assert_eq!(u64::from(s.reserved), state & !0xF, "state {state:#x}");
    }
}
