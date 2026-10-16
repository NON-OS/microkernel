// NONOS Operating System (AGPL-3.0-or-later)
//! Control replies read by what the device sent: a completion cut short
//! of its MessageLength is not an answer whatever the buffer held, a media
//! state indicated while the bind waits is where the link starts, an
//! indication shorter than its header says nothing, and a QUERY buffer
//! inside the completion's own header is no information.

use nonos_usbnet::{Bind, Nic};

use crate::control_tests::bind_with;
use crate::device::{indicate, words};
use crate::qemu::Tamper;
use crate::rndis::from_device::{from_device, FromDevice};
use crate::rndis::query::info;

#[test]
fn a_completion_cut_short_of_its_length_is_passed_over() {
    let cut: Tamper = Box::new(|k, r| match k {
        2 => vec![r[..28].to_vec(), r],
        _ => vec![r],
    });
    assert!(matches!(bind_with(cut).1, Bind::Ours(_)), "the whole one that follows binds");
}

#[test]
fn a_media_state_indicated_during_the_bind_is_where_the_link_starts() {
    let down: Tamper = Box::new(|k, r| match k {
        4 => vec![indicate(0x4001_000C), r],
        _ => vec![r],
    });
    let Bind::Ours(nic) = bind_with(down).1 else { panic!("not bound") };
    assert!(!nic.link_up());
    let Bind::Ours(nic) = bind_with(Box::new(|_, r| vec![r])).1 else { panic!("not bound") };
    assert!(nic.link_up(), "nothing indicated: up, as before");
}

#[test]
fn an_indication_shorter_than_its_header_says_nothing() {
    assert_eq!(from_device(&words(&[7, 12, 0x4001_000C])), FromDevice::Other);
    assert_eq!(from_device(&words(&[7, 16, 0x4001_000C, 0])), FromDevice::Other);
    assert_eq!(from_device(&indicate(0x4001_000C)), FromDevice::Media(false));
}

#[test]
fn a_query_buffer_inside_the_header_is_no_information() {
    let at = |off: u32| words(&[0x8000_0004, 30, 2, 0, 6, off, 0xAABB_CCDD, 0x1122]);
    assert_eq!(info(&at(0)), None, "RequestID and Status as an address");
    assert_eq!(info(&at(15)), None, "one byte into the header");
    assert_eq!(info(&at(16)).map(<[u8]>::len), Some(6), "right after it");
    assert_eq!(info(&words(&[0x8000_0004, 24, 2, 0, 0, 0])), Some(&[][..]), "empty");
}
