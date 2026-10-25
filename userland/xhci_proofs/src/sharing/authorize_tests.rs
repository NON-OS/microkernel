// NONOS Operating System (AGPL-3.0-or-later)
//! The per-device authorization gate: Allow binds as before, Deny binds
//! nothing, Ask holds a HID or several-class configuration until the owner
//! answers, and an answer for another device, or with another ticket,
//! settles nothing (configuration switches: set_config_tests.rs).

use crate::slots::authorize::{Authority, Policy, DEFAULT_POLICY};
use crate::slots::Standing;

/// A configuration of interfaces, each (class, subclass, protocol).
pub(crate) fn config(ifs: &[(u8, u8, u8)]) -> Vec<u8> {
    let total = (9 + 9 * ifs.len()) as u16;
    let mut c = vec![9, 2, total as u8, (total >> 8) as u8, ifs.len() as u8, 1, 0, 0x80, 50];
    for (n, &(class, sub, proto)) in ifs.iter().enumerate() {
        c.extend([9, 4, n as u8, 0, 1, class, sub, proto, 0]);
    }
    c
}

const KEYBOARD: (u8, u8, u8) = (3, 1, 1);
const STORAGE: (u8, u8, u8) = (8, 6, 0x50);

#[test]
fn allow_is_the_default_and_binds_as_before() {
    assert_eq!(DEFAULT_POLICY, Policy::Allow);
    let a = Authority::default();
    assert_eq!(a.admit(4), Some(Ok(())), "nothing read, nothing judged");
}

#[test]
fn deny_binds_nothing() {
    let mut a = Authority::new(Policy::Deny);
    for c in [config(&[STORAGE]), config(&[KEYBOARD])] {
        assert_eq!(a.judge(4, 1, Some(&c)), Standing::Denied);
        assert_eq!(a.admit(4), Some(Err(-13)));
    }
}

#[test]
fn ask_holds_hid_or_several_classes_until_answered() {
    let mut a = Authority::new(Policy::Ask);
    assert_eq!(a.judge(1, 1, Some(&config(&[STORAGE]))), Standing::Authorized);
    let cdc = config(&[(2, 6, 0), (0x0A, 0, 0)]);
    assert_eq!(a.judge(2, 1, Some(&cdc)), Standing::Authorized, "CDC");
    for (slot, c) in [(3, config(&[KEYBOARD])), (4, config(&[(3, 0, 0), STORAGE]))] {
        assert!(matches!(a.judge(slot, 1, Some(&c)), Standing::Held { .. }));
        assert_eq!(a.admit(slot), Some(Err(-13)), "nothing bound while held");
    }
    assert!(matches!(a.judge(5, 1, None), Standing::Held { .. }), "an unread configuration");
    assert!(matches!(a.judge(6, 1, Some(&config(&[]))), Standing::Held { .. }), "no interface");
    assert_eq!(a.held().count(), 4);
}

#[test]
fn an_answer_settles_only_its_own_device() {
    let mut a = Authority::new(Policy::Ask);
    let Standing::Held { ticket } = a.judge(3, 1, Some(&config(&[KEYBOARD]))) else { panic!() };
    a.judge(4, 1, Some(&config(&[KEYBOARD])));
    assert!(!a.answer(4, ticket, true), "another device's ticket");
    assert!(!a.answer(3, ticket.wrapping_add(7), true), "another ticket");
    assert!(!a.answer(9, ticket, true), "a slot never held");
    assert_eq!(a.admit(3), Some(Err(-13)));
    assert!(a.answer(3, ticket, true));
    assert_eq!(a.admit(3), Some(Ok(())));
    assert_eq!(a.admit(4), Some(Err(-13)), "still held");
    a.forget(3);
    assert_eq!(a.admit(3), None, "a new device on the slot is judged afresh");
}
