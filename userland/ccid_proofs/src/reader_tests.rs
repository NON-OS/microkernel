// NONOS Operating System (AGPL-3.0-or-later)
//! Only APDU-level readers with a class descriptor and a bulk pair are
//! taken; the message size is cut to what one transfer carries.

use crate::ccid::find_reader;
use crate::reader::{class, config, EXTENDED, SHORT, TPDU};

#[test]
fn an_apdu_reader_is_found_with_its_slots_and_slot_change_endpoint() {
    let r = find_reader(&config(&class(1, EXTENDED, 271), true)).unwrap().unwrap();
    assert_eq!((r.interface, r.max_slot, r.features, r.max_message), (0, 1, EXTENDED, 271));
    assert_eq!((r.pipes.bulk_in, r.pipes.bulk_out), (0x81, 0x02));
    assert_eq!(r.notify.map(|n| (n.address, n.interval)), Some((0x83, 16)));
    let short = find_reader(&config(&class(0, SHORT, 271), false)).unwrap().unwrap();
    assert_eq!(short.notify, None, "slot changes go unseen, the reader still serves");
}

#[test]
fn tpdu_readers_and_readers_without_the_descriptor_are_left() {
    let tpdu = find_reader(&config(&class(0, TPDU, 271), true));
    assert_eq!(tpdu, Some(Err("TPDU or character level reader")));
    assert_eq!(find_reader(&config(&[], true)), Some(Err("no CCID descriptor")));
    let mut short = class(0, SHORT, 271);
    (short[0], _) = (53, short.pop());
    assert_eq!(find_reader(&config(&short, true)), Some(Err("no CCID descriptor")));
}

#[test]
fn a_huge_message_length_is_cut_to_one_transfer() {
    let r = find_reader(&config(&class(0, EXTENDED, 65_544), true)).unwrap().unwrap();
    assert_eq!(r.max_message, 4096);
}

#[test]
fn another_class_is_not_a_reader() {
    let mut c = config(&class(0, SHORT, 271), true);
    c[9 + 5] = 0x03;
    assert_eq!(find_reader(&c), None);
}
