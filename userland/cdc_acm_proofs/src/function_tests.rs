// NONOS Operating System (AGPL-3.0-or-later)
//! Finding the ACM function: its interfaces, capabilities, bulk pair and
//! notification endpoint, and what is not one.

use crate::acm::{find_acm, Notify, CAP_BREAK, CAP_LINE};
use crate::modem::*;

#[test]
fn a_modem_is_found_with_its_pipes_and_notifications() {
    let f = find_acm(&plain()).expect("an ACM function");
    assert_eq!((f.comm, f.protocol, f.data, f.data_alt), (0, 1, 1, 0));
    assert_eq!(f.capabilities & (CAP_LINE | CAP_BREAK), CAP_LINE | CAP_BREAK);
    assert_eq!((f.pipes.bulk_in, f.pipes.bulk_out), (0x81, 0x02));
    let n = Notify { address: 0x83, max_packet: 16, interval: 10 };
    assert_eq!(f.notify, Some(n));
}

#[test]
fn without_a_notification_endpoint_or_acm_descriptor_it_still_binds() {
    let f = find_acm(&modem(&[&HEADER, &UNION], &[])).expect("bound");
    assert_eq!((f.notify, f.capabilities), (None, 0));
}

#[test]
fn no_union_or_no_data_pair_is_not_an_acm_function() {
    assert!(find_acm(&modem(&[&HEADER, &acm(6)], &ep(0x83, 3, 16, 10))).is_none());
    let mut ecm = plain();
    ecm[9 + 6] = 0x06; // the communications interface's subclass: ECM
    assert!(find_acm(&ecm).is_none());
    let one_way =
        config(&[&iface(0, 0, 0, COMM), &UNION, &iface(1, 0, 1, DATA), &ep(0x81, 2, 64, 0)]);
    assert!(find_acm(&one_way).is_none());
}

#[test]
fn a_bulk_endpoint_on_the_comm_interface_is_not_its_notifications() {
    let f = find_acm(&modem(&[&HEADER, &UNION], &ep(0x83, 0x02, 64, 0))).expect("bound");
    assert_eq!(f.notify, None);
}
