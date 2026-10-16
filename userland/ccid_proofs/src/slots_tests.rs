// NONOS Operating System (AGPL-3.0-or-later)
//! Slot changes read from NotifySlotChange and kept until asked once; the
//! slots powered remembered so they are powered off at the end.

use crate::ccid::{served, Slots};

#[test]
fn a_change_is_kept_until_asked_once() {
    let mut s = Slots::default();
    // Slot 0 present and changed, slot 1 present, slot 2 changed.
    s.seen(&[0x50, 0b0010_0111], 3);
    assert!(s.take_changed(0) && !s.take_changed(0), "asked once");
    assert!(!s.take_changed(1));
    assert!(s.take_changed(2));
    s.seen(&[0x51, 0xFF], 3);
    assert!(!s.take_changed(0), "not a slot-change notice");
    s.seen(&[0x50, 0b0000_0010], 3);
    s.forget_changes();
    assert!(!s.take_changed(0));
}

#[test]
fn slots_past_the_reader_or_the_memory_are_not_marked() {
    let mut s = Slots::default();
    s.seen(&[0x50, 0xFF], 0);
    assert!(s.take_changed(0) && !s.take_changed(1), "only slot 0 is the reader's");
    assert!(!s.take_changed(200));
    s.power(200, true);
    assert_eq!(s.each_powered().count(), 0);
}

#[test]
fn powered_slots_are_listed_until_powered_off() {
    let mut s = Slots::default();
    s.power(0, true);
    s.power(5, true);
    assert_eq!(s.each_powered().collect::<Vec<_>>(), [0, 5]);
    s.power(0, false);
    assert_eq!(s.each_powered().collect::<Vec<_>>(), [5]);
}

#[test]
fn only_slots_the_reader_has_and_the_driver_remembers_are_served() {
    assert!(served(0, 0) && served(63, 255));
    assert!(!served(1, 0), "past the reader's last");
    assert!(!served(64, 255) && !served(255, 255), "past what is remembered");
}
