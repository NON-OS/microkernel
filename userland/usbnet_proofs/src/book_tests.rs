// NONOS Operating System (AGPL-3.0-or-later)
//! Which ports a pass tries.

use crate::scan::{Book, Port, TRIES};

fn port(id: u8, owner: u8, changed: bool) -> Port {
    Port { id, owner, changed }
}

#[test]
fn free_ports_are_tried_and_held_ones_are_not() {
    let mut book = Book::default();
    let plan = book.plan(&[port(1, 0, false), port(2, 2, false), port(3, 1, false)]);
    assert_eq!(plan, vec![1]);
}

#[test]
fn a_port_failing_is_left_after_its_tries() {
    let mut book = Book::default();
    let ports = [port(1, 0, false)];
    for _ in 0..TRIES {
        assert_eq!(book.plan(&ports), vec![1]);
        book.failed(1);
    }
    assert!(book.plan(&ports).is_empty());
}

#[test]
fn another_kind_of_device_is_left_until_it_reconnects() {
    let mut book = Book::default();
    book.plan(&[port(5, 0, false)]);
    book.not_ours(5);
    assert!(book.plan(&[port(5, 0, false)]).is_empty());
    assert_eq!(book.plan(&[port(5, 0, true)]), vec![5], "connect change");
    book.not_ours(5);
    assert!(book.plan(&[]).is_empty());
    assert_eq!(book.plan(&[port(5, 0, false)]), vec![5], "unplugged and back");
}

#[test]
fn retry_all_reopens_every_port() {
    let mut book = Book::default();
    book.plan(&[port(7, 0, false)]);
    book.not_ours(7);
    book.retry_all();
    assert_eq!(book.plan(&[port(7, 0, false)]), vec![7]);
}
