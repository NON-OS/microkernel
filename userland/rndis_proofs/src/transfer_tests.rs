// NONOS Operating System (AGPL-3.0-or-later)
//! Frames through a bound QEMU usb-net: each one out as a PACKET_MSG that
//! QEMU's usb_net_handle_dataout reads back whole, a batched bulk IN
//! handed up one frame per receive, and a stall cleared on both sides.

use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::{Bind, Nic, Setup};

use crate::control_tests::bind_with;
use crate::device::{le, packet, qemu_takes};
use crate::qemu::honest;
use crate::rndis::link::Rndis;

fn bound() -> (MockBus, Rndis<MockBus>) {
    let (bus, r) = bind_with(honest());
    let Bind::Ours(nic) = r else { panic!("not bound") };
    (bus, nic)
}

fn last_out(bus: &MockBus) -> Vec<u8> {
    match bus.0.borrow().calls.last() {
        Some(Call::BulkOut(b)) => b.clone(),
        other => panic!("{other:?}"),
    }
}

#[test]
fn each_frame_is_one_message_qemu_reads_whole_with_the_padding_inside() {
    let (bus, mut nic) = bound();
    for (len, transfer) in [(60, 104), (20, 65), (84, 129), (1514, 1558), (14, 58)] {
        let frame: Vec<u8> = (0..len).map(|i| i as u8).collect();
        nic.send(&frame).unwrap();
        let out = last_out(&bus);
        assert_eq!((out.len(), le(&out, 4) as usize), (transfer, transfer), "frame {len}");
        assert_eq!((le(&out, 8), le(&out, 12) as usize), (36, len));
        assert_eq!(qemu_takes(&out), (Some(frame), 0), "nothing left for the next message");
    }
    assert_eq!(nic.send(&[0; 13]), Err(-22));
    assert_eq!(nic.send(&[0; 1515]), Err(-22));
}

#[test]
fn a_batched_transfer_goes_up_one_frame_per_receive() {
    let (bus, mut nic) = bound();
    let (a, b) = (vec![0xA5u8; 98], vec![0x5Au8; 1514]);
    let t = [packet(&a), packet(&b), packet(&a)].concat();
    bus.0.borrow_mut().bulk_in.extend([Ok(Some(t)), Ok(None)]);
    let mut out = [0u8; 1514];
    for want in [&a, &b, &a] {
        let n = nic.recv(&mut out).unwrap().unwrap();
        assert_eq!(&out[..n], &want[..]);
    }
    assert_eq!(nic.recv(&mut out), Ok(None));
    assert!(bus.0.borrow().bulk_in.is_empty(), "the pipe is read only once the queue is empty");
    bus.0.borrow_mut().bulk_in.push_back(Ok(Some(packet(&b))));
    assert_eq!(nic.recv(&mut [0u8; 100]), Ok(None), "a frame past the buffer is dropped");
}

#[test]
fn a_stalled_pipe_is_cleared_on_both_sides() {
    let (bus, mut nic) = bound();
    bus.0.borrow_mut().bulk_in.push_back(Err(-32));
    assert_eq!(nic.recv(&mut [0u8; 1514]), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    let n = calls.len();
    assert_eq!(calls[n - 2], Call::ResetBulk(true));
    assert_eq!(calls[n - 1], Call::Out(Setup::new(0x02, 0x01, 0, 0x82), vec![]));
    bus.0.borrow_mut().bulk_out_fails = Some(-32);
    assert_eq!(nic.send(&[1; 60]), Err(-32));
    let last = bus.0.borrow().calls.last().cloned();
    assert_eq!(last, Some(Call::Out(Setup::new(0x02, 0x01, 0, 0x02), vec![])));
}
