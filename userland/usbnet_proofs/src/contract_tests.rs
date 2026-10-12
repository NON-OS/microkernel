// NONOS Operating System (AGPL-3.0-or-later)
//! The NNET contract as net.core holds it, read from net.core's own
//! source: the same numbers for the same operations, and no answer to an
//! operation net.core numbers that it could read as another.

use crate::fake_nic::Fake;
use crate::nnet::{answer, decode, Request, Stats, DATA_AT, MAGIC, STATS_LEN, VERSION};
use crate::nnet::{OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_STATS, OP_TX_PACKET};

/// net.core's numbers, its DHCP lease ops included.
#[path = "../../capsule_net_core/src/protocol/ops.rs"]
mod net_core;

fn ask(op: u16, body: &[u8]) -> (i32, Vec<u8>) {
    let mut nic = Fake { sent: vec![], frames: vec![Ok(None)] };
    let req = Request { op, flags: 0, request_id: 3, payload_len: body.len() as u32 };
    let mut tx = vec![0u8; 2048];
    let n = answer(Some(&mut nic), &mut Stats::default(), &req, body, &mut tx);
    (i32::from_le_bytes(tx[20..24].try_into().unwrap()), tx[DATA_AT..n].to_vec())
}

#[test]
fn every_operation_net_core_sends_has_its_number_here() {
    assert_eq!((net_core::MAGIC_NNET, net_core::VERSION), (MAGIC, VERSION));
    assert_eq!(net_core::OP_LINK_STATUS, OP_LINK_STATUS);
    assert_eq!(net_core::OP_MAC_ADDRESS, OP_MAC_ADDRESS);
    assert_eq!(net_core::OP_TX_PACKET, OP_TX_PACKET);
    assert_eq!(net_core::OP_RX_PACKET, OP_RX_PACKET);
}

#[test]
fn the_counters_answer_on_a_number_net_core_never_sends() {
    let theirs = [net_core::OP_LINK_STATUS, net_core::OP_MAC_ADDRESS, net_core::OP_TX_PACKET];
    let theirs = [&theirs[..], &[net_core::OP_RX_PACKET, net_core::OP_RX_BATCH]].concat();
    assert!(!theirs.contains(&OP_STATS), "{OP_STATS:#x}");
    let (status, data) = ask(OP_STATS, &[]);
    assert_eq!((status, data.len()), (0, STATS_LEN));
}

#[test]
fn the_receive_batch_probe_is_answered_as_not_served() {
    for body in [&[][..], &7u32.to_le_bytes()[..]] {
        let (status, data) = ask(net_core::OP_RX_BATCH, body);
        assert!(status != 0 || data == [0u8; 4], "status {status}, {} bytes", data.len());
    }
}

#[test]
fn a_lease_query_under_net_core_s_other_magic_is_never_read_as_ours() {
    let mut req = vec![0u8; net_core::HDR_LEN];
    req[..4].copy_from_slice(&net_core::MAGIC_NDHC.to_le_bytes());
    req[4..6].copy_from_slice(&net_core::VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&net_core::OP_LEASE_STATUS.to_le_bytes());
    assert_eq!(net_core::OP_LEASE_STATUS, OP_MAC_ADDRESS);
    assert!(decode(&req).is_none());
}
