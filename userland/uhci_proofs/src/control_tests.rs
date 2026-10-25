// NONOS Operating System (AGPL-3.0-or-later)
//! EP0 on the wire: descriptors and any control transfer, at both speeds.

use crate::model::{addressed, call, rig, Device};
use crate::wire::ops::*;

#[test]
fn the_device_descriptor_comes_whole_at_8_and_64_byte_packets() {
    for (dev, port) in [(Device::low(), 1u8), (Device::full(64), 1)] {
        let want = dev.desc;
        let (mut cx, w) = rig([Some(dev), None]);
        let slot = addressed(&mut cx, port);
        let r = call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[slot]);
        assert_eq!((r.status, r.data.as_slice()), (0, &want[..]));
        assert_eq!(w.borrow().devs[0].as_ref().unwrap().toggle_errors, 0);
    }
}

#[test]
fn the_config_descriptor_answers_its_length_then_the_bytes() {
    let dev = Device::full(8);
    let config = dev.config.clone();
    let (mut cx, _) = rig([Some(dev), None]);
    let slot = addressed(&mut cx, 1);
    let head = call(&mut cx, OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, 9, 0]);
    assert_eq!(&head.data[..4], &[9, 0, 0, 0]);
    let n = config.len() as u8;
    let all = call(&mut cx, OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, n, 0]);
    assert_eq!(&all.data[..4], &[n, 0, 0, 0]);
    assert_eq!(&all.data[4..], &config[..]);
}

#[test]
fn a_short_in_data_stage_ends_early_and_status_still_runs() {
    let dev = Device::full(8);
    let n = dev.config.len() as u8;
    let (mut cx, w) = rig([Some(dev), None]);
    let slot = addressed(&mut cx, 1);
    let r = call(&mut cx, OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, 255, 0]);
    assert_eq!((r.status, r.data[0]), (0, n), "asked 255, the device had {n}");
    let again = call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[slot]);
    assert_eq!(again.status, 0, "EP0 is not left mid-request");
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().toggle_errors, 0);
}

#[test]
fn an_out_request_carries_its_data_and_an_in_request_returns_its_own() {
    let (mut cx, w) = rig([Some(Device::low()), None]);
    let slot = addressed(&mut cx, 1);
    let mut out = vec![slot, 0, 0x21, 0x09, 0, 2, 0, 0, 20, 0];
    out.extend(1..=20u8);
    let r = call(&mut cx, OP_CONTROL_TRANSFER, &out);
    assert_eq!((r.status, &r.data[..2]), (0, &[20u8, 0][..]));
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().ctrl_out, (1..=20).collect::<Vec<u8>>());
    let r = call(&mut cx, OP_CONTROL_TRANSFER, &[slot, 0, 0xC0, 1, 0, 0, 0, 0, 30, 0]);
    assert_eq!(&r.data[..2], &[30, 0]);
    assert_eq!(&r.data[2..], &(0..30).collect::<Vec<u8>>()[..]);
    let r = call(&mut cx, OP_CONTROL_TRANSFER, &[slot, 0, 0x00, 9, 1, 0, 0, 0, 0, 0]);
    assert_eq!((r.status, r.data.as_slice()), (0, &[0u8, 0][..]), "no data stage");
}
