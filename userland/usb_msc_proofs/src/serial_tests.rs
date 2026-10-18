// NONOS Operating System (AGPL-3.0-or-later)
//! The serial number (disk/serial.rs) as the device's descriptors give it:
//! read to its own length and no further, whatever length it claims, and
//! kept only as a digest that tells two sticks apart. One that will not
//! read is none: the stick is bound all the same, and served no more after
//! a let-go. A failed read refused the stick.

use crate::disk::serial;
use crate::fixture::{disk, ready_unit, sense};
use crate::protocol::E_IO;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{read, until_ready};
use crate::sticks::plug;
use crate::xhci::{script, setup, Answer};

/// The digest of the serial number of the device on port 1, whose string
/// descriptor is `descriptor`, or which has none.
fn digest(descriptor: Option<Vec<u8>>) -> Option<u64> {
    script(vec![]);
    let serials = descriptor.map(|s| vec![(1, s)]).unwrap_or_default();
    setup(|d| (d.addressed, d.serials) = (vec![1], serials));
    serial(&disk())
}

#[test]
fn a_serial_number_tells_two_sticks_apart_by_its_own_bytes_alone() {
    let nonos = digest(Some(vec![8, 3, b'N', 0, b'O', 0, b'N', 0]));
    assert!(nonos.is_some());
    assert_ne!(nonos, digest(Some(vec![8, 3, b'O', 0, b'T', 0, b'H', 0])));
    let trailing = vec![8, 3, b'N', 0, b'O', 0, b'N', 0, 0xEE, 0xEE];
    assert_eq!(nonos, digest(Some(trailing)), "bytes past its length do not count");
    assert_eq!(digest(None), None);
    for empty in [vec![0, 3, b'N', 0], vec![2, 3, b'N', 0]] {
        assert_eq!(digest(Some(empty)), None, "a length that holds no character names none");
    }
}

#[test]
fn any_length_a_serial_number_claims_is_held_to_the_bytes_that_came() {
    for claimed in [1, 2, 3, 254, 255] {
        for sent in [0, 1, 2, 9, 255, 300] {
            let mut descriptor = vec![claimed, 3];
            descriptor.resize(sent, 0xA5);
            let mut counted = descriptor.clone();
            counted.resize(usize::from(claimed), 0);
            assert_eq!(digest(Some(descriptor)), digest(Some(counted)), "{claimed} of {sent}");
        }
    }
}

#[test]
fn a_stick_whose_serial_number_will_not_read_is_bound_and_served_once() {
    let ok = || Answer::Csw { residue: 0, status: 0 };
    let fails =
        |key, asc| [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, asc)), ok()];
    plug(ready_unit(1000));
    setup(|d| (d.strings_fail, d.addressed) = (Some(E_IO), vec![1]));
    assert_eq!(serial(&disk()), None, "a serial number that will not read is none");
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    let check = fails(0x02, 0x04).into_iter().chain([ok()]).chain(ready_unit(1000));
    setup(|d| d.answers = fails(0x03, 0x11).into_iter().chain(check).collect());
    assert_eq!(read(&mut server), E_IO);
    assert!(matches!(server.scanner.medium, Medium::Changed), "nothing tells it from another");
}
