// NONOS Operating System (AGPL-3.0-or-later)
//! Well-formed NTB16s in: several NDPs and several datagrams per block,
//! handed up one per call from the queue the driver keeps.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Bind, Nic, Setup};

use crate::device::{calls, Dev};
use crate::ncm::bind;
use crate::ntb::{found, Ntb, NCM0, NCM1};

#[test]
fn two_ndps_with_two_datagrams_each_come_up_in_order() {
    let mut b = Ntb::new(400, 12);
    b.ndp(12, NCM0, 200, &[(40, 60), (100, 64)]).ndp(200, NCM0, 0, &[(240, 14), (300, 98)]);
    b.datagram(40, 60, 1).datagram(100, 64, 2).datagram(240, 14, 3).datagram(300, 98, 4);
    assert_eq!(found(&b.0, 4096), [(40, 60), (100, 64), (240, 14), (300, 98)]);
    let dev = Dev::ncm(0);
    let bus = dev.bus();
    let Bind::Ours(mut nic) = bind(bus.clone(), &dev.found()) else { panic!("not bound") };
    bus.0.borrow_mut().bulk_in.extend([Ok(Some(b.0.clone())), Ok(None)]);
    let mut out = [0u8; 1514];
    for (len, fill) in [(60, 1), (64, 2), (14, 3), (98, 4)] {
        assert_eq!(nic.recv(&mut out), Ok(Some(len)));
        assert!(out[..len].iter().all(|&x| x == fill));
    }
    assert_eq!(nic.recv(&mut out), Ok(None), "the queue is empty and the pipe idle");
}

#[test]
fn a_crc_ndp_is_passed_over_and_the_chain_followed() {
    let mut b = Ntb::new(300, 12);
    b.ndp(12, NCM1, 100, &[(40, 60)]).ndp(100, NCM0, 0, &[(200, 60)]);
    assert_eq!(found(&b.0, 4096), [(200, 60)]);
}

#[test]
fn an_ndp_may_follow_its_datagrams_and_the_first_null_entry_ends_it() {
    let mut b = Ntb::new(200, 140);
    b.ndp(140, NCM0, 0, &[(12, 60), (0, 0), (80, 60)]);
    assert_eq!(found(&b.0, 4096), [(12, 60)], "entries after the null one are ignored");
}

#[test]
fn a_datagram_longer_than_the_stack_takes_is_dropped_and_the_next_served() {
    let mut b = Ntb::new(2000, 12);
    b.ndp(12, NCM0, 0, &[(32, 1600), (1700, 60)]);
    let dev = Dev::ncm(0);
    let bus = dev.bus();
    let Bind::Ours(mut nic) = bind(bus.clone(), &dev.found()) else { panic!("not bound") };
    bus.0.borrow_mut().bulk_in.push_back(Ok(Some(b.0)));
    let mut out = [0u8; 1514];
    assert_eq!(nic.recv(&mut out), Ok(Some(60)));
}

#[test]
fn a_stalled_in_pipe_is_cleared_on_both_sides() {
    let dev = Dev::ncm(0);
    let bus = dev.bus();
    let Bind::Ours(mut nic) = bind(bus.clone(), &dev.found()) else { panic!("not bound") };
    bus.0.borrow_mut().bulk_in.push_back(Err(-32));
    assert_eq!(nic.recv(&mut [0u8; 1514]), Err(-32));
    let c = calls(&bus);
    assert_eq!(c[c.len() - 2], Call::ResetBulk(true));
    assert_eq!(c[c.len() - 1], Call::Out(Setup::new(0x02, 0x01, 0, 0x82), vec![]));
}
