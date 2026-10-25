// NONOS Operating System (AGPL-3.0-or-later)
//! net.core asks a card for OP_RX_BATCH before anything else and binds the
//! batch path when the card answers it with status 0. On the old wire these
//! drivers answered that op with their counters, so net.core took the
//! counters for frames and never asked for one frame at a time.

use crate::nic::Nic;
use crate::nnet::{answer, decode, Request, Stats, E_INVAL, OP_STATS, STATS_LEN};
use crate::station::Taken;
use nonos_mac::resend::Kept;

struct Unbound;

impl Nic for Unbound {
    fn mac(&self) -> [u8; 6] {
        [0; 6]
    }
    fn taken(&self) -> Taken {
        Taken::Unconfirmed
    }
    fn link_up(&self) -> bool {
        false
    }
    fn send(&mut self, _: &[u8]) -> Result<(), i32> {
        Err(E_INVAL)
    }
    fn recv(&mut self, _: &mut [u8]) -> Result<Option<usize>, i32> {
        Ok(None)
    }
}

fn status(op: u16) -> (i32, usize) {
    let req = Request { op, flags: 0, request_id: 3, payload_len: 4 };
    let mut tx = vec![0u8; 256];
    let n = answer::<Unbound>(
        None,
        (&mut Stats::default(), &mut Kept::new()),
        &req,
        &1u32.to_le_bytes(),
        &mut tx,
    );
    assert_eq!(decode(&tx[..n]).map(|r| r.op), Some(op));
    (i32::from_le_bytes(tx[20..24].try_into().unwrap()), n - 24)
}

#[test]
fn net_cores_batch_op_is_refused_and_the_counters_are_past_it() {
    let net_core = include_str!("../../capsule_net_core/src/protocol/ops.rs");
    let batch = net_core
        .lines()
        .find_map(|l| l.strip_prefix("pub const OP_RX_BATCH: u16 = "))
        .and_then(|v| v.trim_end_matches(';').parse::<u16>().ok())
        .expect("net.core names its batch op");
    assert_ne!(OP_STATS, batch, "the counters answer net.core's batch op");
    assert_eq!(status(batch).0, E_INVAL, "the batch op answered with status 0");
    assert_eq!(status(OP_STATS), (0, STATS_LEN), "the counters on their own op");
}
