// NONOS Operating System (AGPL-3.0-or-later)
//! The receive path held to NCM 1.0's own structure: an NTB32 only from a
//! device that offered it, a header of its format's own length, an NTB
//! that all arrived and is read no further than its block length, and NTB
//! parameters no shorter than their own wLength says.

use std::collections::VecDeque;

use nonos_usbnet::Bind;

use crate::device::Dev;
use crate::ncm::bind;
use crate::ncm::ntb_in::datagrams;
use crate::ncm::params::parse_params;
use crate::ntb::{found, Ntb, NCM0};
use crate::ntb32_tests::{ndp32, ntb32, NDP32};
use crate::spec::ntb_params;

fn found_as(raw: &[u8], ntb32: bool) -> Vec<(usize, usize)> {
    let mut q = VecDeque::new();
    datagrams(raw, 4096, ntb32, &mut q);
    q.into_iter().collect()
}

#[test]
fn an_ntb32_is_read_only_from_a_device_that_offered_it() {
    let mut b = ntb32(1024, 16);
    ndp32(&mut b, 16, NDP32, 0, &[(200, 60)]);
    assert_eq!(found_as(&b, true), [(200, 60)]);
    assert_eq!(found_as(&b, false), [], "16-bit only");
    let offered = |formats| {
        let mut dev = Dev::ncm(0);
        dev.params = ntb_params(formats, 16384, 16384, (4, 0, 4));
        let Bind::Ours(nic) = bind(dev.bus(), &dev.found()) else { panic!("not bound") };
        nic.ntb32
    };
    assert_eq!((offered(1), offered(3)), (false, true));
}

#[test]
fn a_header_of_the_other_formats_length_gives_nothing() {
    let mut b = Ntb::new(128, 12);
    b.ndp(12, NCM0, 0, &[(40, 60)]);
    assert_eq!(found(&b.u16(4, 16).0, 4096), [], "NTH16 claiming 16 bytes");
    let mut c = ntb32(1024, 16);
    ndp32(&mut c, 16, NDP32, 0, &[(200, 60)]);
    c[4..6].copy_from_slice(&12u16.to_le_bytes());
    assert_eq!(found(&c, 4096), [], "NTH32 claiming 12 bytes");
}

#[test]
fn nothing_past_the_block_length_is_read_and_a_cut_block_gives_nothing() {
    let mut b = Ntb::new(256, 12);
    b.ndp(12, NCM0, 0, &[(40, 60), (150, 60)]).u16(8, 128);
    assert_eq!(found(&b.0, 4096), [(40, 60)], "the second datagram lies past the block");
    b.ndp(12, NCM0, 200, &[(40, 60)]).ndp(200, NCM0, 0, &[(140, 20)]);
    assert_eq!(found(&b.0, 4096), [(40, 60)], "an NDP past the block");
    assert_eq!(found(&b.u16(8, 300).0, 4096), [], "the block did not all arrive");
    assert_eq!(found(&b.u16(8, 0).0, 4096).len(), 2, "no length: the transfer's end");
}

#[test]
fn ntb_parameters_shorter_than_their_own_wlength_say_are_refused() {
    let mut p = ntb_params(1, 4096, 4096, (4, 0, 4));
    assert!(parse_params(&p).is_some());
    p[0] = 0x18;
    assert_eq!(parse_params(&p), None);
    p[0] = 0x20;
    assert!(parse_params(&p).is_some(), "a longer structure: its first 28 bytes");
}
