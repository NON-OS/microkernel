// NONOS Operating System (AGPL-3.0-or-later)
//! driver.usb_hub0's pass over the root ports (nonos_usb_hub's roots)
//! through the real host front: a hub kept, any other device given back
//! for the class drivers, and a hub leaving its root port taking every
//! child along, a claimed one reaching its claimant as a detach.

use nonos_usb_hub::{roots, scan_hubs, Hub, Parker, Root};

use crate::hub_lane::Lane;
use crate::hub_seen::hub_dev;
use crate::model::hub::HubModel;
use crate::model::{call_host, rig, Cx, Device};
use crate::server::many::Front;
use crate::wire::ops::*;

fn listed(f: &mut Front<Cx>) -> (Vec<Root>, Vec<u8>) {
    let r = call_host(f, OP_PORT_STATUS, &[]);
    let e: Vec<&[u8]> = r.data[4..4 + 8 * r.data[0] as usize].chunks(8).collect();
    let sc = |e: &[u8]| u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
    let root = e.iter().filter(|e| e[2] == 0).map(|e| (e[0], e[1], sc(e))).collect();
    (root, e.iter().filter(|e| e[2] == 1).map(|e| e[0]).collect())
}

fn pass(f: &mut Front<Cx>, hubs: &mut Vec<Hub>, tried: &mut [u8; 256]) {
    let (root, _) = listed(f);
    let mut lane = Lane { cx: f, lines: vec![] };
    roots(&mut lane, &root, hubs, tried);
    scan_hubs(&mut lane, hubs, &mut Parker);
}

#[test]
fn a_hub_is_kept_a_stick_given_back_and_the_hub_leaving_takes_all() {
    let hub = HubModel::new(false, 4).plug(1, Device::low(), 2).plug(4, Device::full(64), 1);
    let (cx, w) = rig([Some(hub_dev(hub)), Some(Device::full(64))]);
    let mut f = Front::new();
    assert!(f.add(cx));
    let (mut hubs, mut tried) = (vec![], [0u8; 256]);
    pass(&mut f, &mut hubs, &mut tried);
    assert_eq!((hubs.len(), hubs[0].place.root_port), (1, 1));
    let (root, virt) = listed(&mut f);
    assert_eq!((root[1].1, tried[2]), (0, 3), "port 2's stick given back, left alone");
    assert_eq!(virt, vec![0x80, 0x81], "the hub's two children parked");
    let s = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call_host(&mut f, OP_ADDRESS_DEVICE, &[s, 0x81]).status, 0, "claimed");
    {
        use crate::facts::io::{PORT_CCS, PORT_CSC, PORT_PE};
        let mut w = w.borrow_mut();
        w.devs[0] = None;
        w.portsc[0] = w.portsc[0] & !(PORT_CCS | PORT_PE) | PORT_CSC;
    }
    pass(&mut f, &mut hubs, &mut tried);
    assert!(hubs.is_empty(), "the hub is forgotten");
    let r = call_host(&mut f, OP_PORT_STATUS, &[]);
    let n = r.data[0] as usize;
    let left: Vec<(u8, u8)> = r.data[4..4 + 8 * n].chunks(8).map(|e| (e[0], e[4] & 1)).collect();
    assert_eq!(left, vec![(1, 0), (2, 1), (0x81, 0)], "only the claimed child, disconnected");
    assert_eq!(call_host(&mut f, OP_DISABLE_SLOT, &[s]).status, 0);
    assert_eq!(listed(&mut f).1, Vec::<u8>::new());
}
