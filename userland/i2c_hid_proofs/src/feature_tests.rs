// NONOS Operating System (AGPL-3.0-or-later)
//! A pad's input mode and switches through a modelled port: written only
//! where a bit is wrong, other bits kept, built from the descriptor when
//! the device will not say.

use std::{cell::RefCell, collections::BTreeMap};

use crate::hid::configure_reporting;
use crate::hid::input_mode::port::{FeaturePort, REPORT_MAX};
use crate::hid::{Field, TouchLayout};

#[derive(Default)]
struct Pad {
    held: RefCell<BTreeMap<u8, Vec<u8>>>,
    writes: RefCell<Vec<Vec<u8>>>,
    silent: bool,
}

impl FeaturePort for Pad {
    fn get(&self, id: u8, report: &mut [u8; REPORT_MAX]) -> usize {
        let r = self.held.borrow().get(&id).filter(|_| !self.silent).cloned().unwrap_or_default();
        report[..r.len()].copy_from_slice(&r);
        r.len()
    }
    fn set(&self, report: &[u8]) -> bool {
        self.held.borrow_mut().insert(report[0], report.to_vec());
        self.writes.borrow_mut().push(report.to_vec());
        true
    }
}

/// Input mode a byte in report 3; surface and button switches bits 0 and 1
/// of report 4.
fn layout() -> TouchLayout {
    let f = |bit_offset, bit_size| Field { bit_offset, bit_size, ..Field::default() };
    let mut l = TouchLayout::default();
    (l.input_mode, l.input_mode_report_id) = (f(0, 8), 3);
    (l.surface_switch, l.surface_switch_report_id) = (f(0, 1), 4);
    (l.button_switch, l.button_switch_report_id) = (f(1, 1), 4);
    l
}

fn pad(mode: u8, switches: u8) -> Pad {
    let p = Pad::default();
    p.held.borrow_mut().extend([(3, vec![3, mode]), (4, vec![4, switches])]);
    p
}

#[test]
fn a_pad_in_mouse_mode_is_switched_with_its_other_bits_kept() {
    let p = pad(0, 0b1111_0000);
    assert!(!configure_reporting(&p, &layout()).refused);
    assert_eq!(*p.writes.borrow(), vec![vec![3, 3], vec![4, 0b1111_0011]]);
}

#[test]
fn a_pad_already_right_is_not_written() {
    let p = pad(3, 0b11);
    assert!(!configure_reporting(&p, &layout()).refused);
    assert!(p.writes.borrow().is_empty());
}

#[test]
fn a_pad_that_will_not_say_gets_a_report_built_from_its_descriptor() {
    let p = Pad { silent: true, ..pad(0, 0) };
    assert!(!configure_reporting(&p, &layout()).refused);
    assert_eq!(*p.writes.borrow(), vec![vec![3, 3], vec![4, 0b11]]);
}

#[test]
fn a_pad_without_the_features_is_left_alone() {
    let p = pad(0, 0);
    assert!(!configure_reporting(&p, &TouchLayout::default()).refused);
    assert!(p.writes.borrow().is_empty());
}
