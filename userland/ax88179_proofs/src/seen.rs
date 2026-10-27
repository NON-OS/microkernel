// NONOS Operating System (AGPL-3.0-or-later)
//! What a bus saw, in the model's terms; what a bind said; transfers fed
//! to a bound chip and the frames that came up.

use nonos_usbnet::mock::{drawn, Call, MockBus};
use nonos_usbnet::{Bind, Nic};

use crate::ax::bind;
use crate::ax::link::{Ax, SPANS_MAX};
use crate::chip::healthy;
use crate::harness::device_of;
use crate::model::Want;

pub fn want(c: &Call) -> Option<Want> {
    match c {
        Call::Out(s, d) if s.request_type == 0x40 => {
            Some(Want::Out(s.request, s.value, s.index, d.clone()))
        }
        Call::In(s, n) if s.request_type == 0xC0 => Some(Want::In(s.request, s.value, s.index, *n)),
        _ => None,
    }
}

/// Every vendor request the bus saw, in order.
pub fn vendor(bus: &MockBus) -> Vec<Want> {
    bus.0.borrow().calls.iter().filter_map(want).collect()
}

/// The vendor requests among the first `n` calls.
pub fn vendor_before(bus: &MockBus, n: usize) -> usize {
    bus.0.borrow().calls[..n].iter().filter_map(want).count()
}

/// PHY status reads the bus saw.
pub fn phy_reads(bus: &MockBus) -> usize {
    vendor(bus).iter().filter(|w| matches!(w, Want::In(2, 3, 0x11, 2))).count()
}

pub fn verdict(device: [u8; 18], configs: Vec<Vec<u8>>) -> (&'static str, Vec<Call>) {
    let (bus, found, _) = device_of(device, configs, healthy());
    let v = match bind(bus.clone(), &found, drawn()) {
        Bind::Ours(_) => "ours",
        Bind::NotOurs => "not ours",
        Bind::Failed(what, _) => what,
    };
    let calls = bus.0.borrow().calls.clone();
    (v, calls)
}

pub fn feed(bus: &MockBus, t: Vec<u8>) {
    bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t)));
}

pub fn drain(ax: &mut Ax<MockBus>) -> Vec<Vec<u8>> {
    let mut got = vec![];
    let mut out = [0u8; 1514];
    for _ in 0..SPANS_MAX + 8 {
        if let Some(n) = ax.recv(&mut out).unwrap() {
            got.push(out[..n].to_vec());
        }
    }
    got
}
