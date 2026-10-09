// NONOS Operating System (AGPL-3.0-or-later)
//! Frames out and in: the padding byte, the receive limits, a stall.

use nonos_usbnet::found::fetch;
use nonos_usbnet::mock::Call;
use nonos_usbnet::{Bind, Nic, Setup};

use crate::ecm::bind;
use crate::ecm::frame::{padded_len, received_len};
use crate::qemu::usb_net;

#[test]
fn a_frame_filling_its_last_packet_gets_one_byte_more() {
    for (len, mps, out) in [(60, 64, 60), (64, 64, 65), (512, 512, 513), (1024, 1024, 1025)] {
        assert_eq!(padded_len(len, mps), out);
    }
    assert_eq!(padded_len(1514, 512), 1514);
    assert_eq!(padded_len(128, 512), 129, "a multiple of 64 on a high-speed pipe");
    assert_eq!(padded_len(72, 8), 73);
}

#[test]
fn a_runt_is_dropped_and_a_padded_frame_cut_to_size() {
    assert_eq!(received_len(13), None);
    assert_eq!(received_len(14), Some(14));
    assert_eq!(received_len(1515), Some(1514));
}

#[test]
fn frames_cross_and_a_stall_is_cleared_on_both_sides() {
    let bus = usb_net(None);
    let found = fetch(&mut bus.clone(), 2).unwrap();
    let Bind::Ours(mut nic) = bind(bus.clone(), &found) else { panic!("not bound") };
    nic.send(&[7u8; 64]).unwrap();
    let last = bus.0.borrow().calls.last().cloned();
    assert_eq!(last, Some(Call::BulkOut([&[7u8; 64][..], &[0]].concat())));
    bus.0.borrow_mut().bulk_in.extend([Ok(Some(vec![9; 60])), Ok(Some(vec![1; 5])), Err(-32)]);
    let mut out = [0u8; 1514];
    assert_eq!(nic.recv(&mut out), Ok(Some(60)));
    assert_eq!(nic.recv(&mut out), Ok(None), "runt");
    assert_eq!(nic.recv(&mut out), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    let n = calls.len();
    assert_eq!(calls[n - 2], Call::ResetBulk(true));
    assert_eq!(calls[n - 1], Call::Out(Setup::new(0x02, 0x01, 0, 0x82), vec![]));
    assert_eq!(nic.recv(&mut out), Ok(None));
}
