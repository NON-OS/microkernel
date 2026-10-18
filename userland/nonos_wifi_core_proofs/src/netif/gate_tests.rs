// NONOS Operating System (AGPL-3.0-or-later)

//! The link protocol is net.core's alone (netif::gate): every op from any
//! other sender is refused in the link's own shape, and nothing but a link
//! frame is ever taken for one, so the control family the panels speak is
//! not caught by the gate.

use nonos_wifi_core::netif::gate::{is_link, link_admits, refuse_link, STATUS_NOT_PERMITTED};
use nonos_wifi_core::netif::wire::{
    parse_request, HDR_LEN, MAGIC_NNET, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_TX_PACKET,
    VERSION,
};

const NET_CORE: u32 = 41;

fn request(op: u16, rid: u32) -> Vec<u8> {
    let mut v = vec![0u8; HDR_LEN + 8];
    v[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    v[4..6].copy_from_slice(&VERSION.to_le_bytes());
    v[6..8].copy_from_slice(&op.to_le_bytes());
    v[12..16].copy_from_slice(&rid.to_le_bytes());
    v[16..20].copy_from_slice(&8u32.to_le_bytes());
    v
}

#[test]
fn only_net_cores_owner_is_admitted() {
    assert!(link_admits(NET_CORE, Some(NET_CORE)));
    for sender in [0, 1, 7, NET_CORE + 1, u32::MAX] {
        assert!(!link_admits(sender, Some(NET_CORE)), "{sender}");
    }
    // No net.core registered, or a kernel sender: nobody.
    assert!(!link_admits(NET_CORE, None) && !link_admits(0, Some(0)) && !link_admits(0, None));
}

#[test]
fn every_link_op_is_refused_in_its_own_shape() {
    for op in [OP_LINK_STATUS, OP_MAC_ADDRESS, OP_TX_PACKET, OP_RX_PACKET] {
        let req = request(op, 0xA1B2_C3D4);
        assert!(is_link(&req), "op {op}");
        let mut out = [0u8; 64];
        let n = refuse_link(&req, &mut out);
        assert_eq!(n, HDR_LEN + 4, "op {op}");
        let head = parse_request(&out[..n]).expect("a well-formed reply header");
        assert_eq!((head.op, head.request_id), (op, 0xA1B2_C3D4));
        let status = i32::from_le_bytes(out[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
        assert_eq!(status, STATUS_NOT_PERMITTED);
    }
}

#[test]
fn nothing_but_a_link_frame_is_gated() {
    // The Wi-Fi control family's tag (nonos_wifi_client driver/call.rs) and
    // a driver's own are not link frames.
    for magic in [0x5749_4649u32, 0x4957_4C46, 0] {
        let mut req = request(OP_TX_PACKET, 1);
        req[0..4].copy_from_slice(&magic.to_le_bytes());
        assert!(!is_link(&req), "{magic:#x}");
    }
    assert!(!is_link(&[]) && !is_link(&MAGIC_NNET.to_le_bytes()[..3]));
    let mut tiny = [0u8; 8];
    assert_eq!(refuse_link(&request(OP_TX_PACKET, 1), &mut tiny), 0);
}
