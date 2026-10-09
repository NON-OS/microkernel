// NONOS Operating System (AGPL-3.0-or-later)
//! The NNET answers net.core and net.l2 get, bound and unbound.

use crate::nic::Nic;
use crate::nnet::{answer, decode, Request, Stats, DATA_AT, E_AGAIN, E_INVAL, E_MSGSIZE};
use crate::nnet::{OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_TX_PACKET};

struct Fake {
    sent: Vec<Vec<u8>>,
    frames: Vec<Result<Option<Vec<u8>>, i32>>,
}

impl Nic for Fake {
    fn mac(&self) -> [u8; 6] {
        [2, 0, 0, 0, 0, 1]
    }
    fn link_up(&self) -> bool {
        true
    }
    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        self.sent.push(frame.to_vec());
        Ok(())
    }
    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        match self.frames.remove(0)? {
            Some(f) => Ok(Some({
                out[..f.len()].copy_from_slice(&f);
                f.len()
            })),
            None => Ok(None),
        }
    }
}

fn ask(nic: Option<&mut Fake>, op: u16, body: &[u8]) -> (i32, Vec<u8>) {
    let req = Request { op, flags: 0, request_id: 9, payload_len: body.len() as u32 };
    let mut tx = vec![0u8; 2048];
    let n = answer(nic, &mut Stats::default(), &req, body, &mut tx);
    assert_eq!(decode(&tx[..n]).map(|r| (r.op, r.request_id)), Some((op, 9)));
    (i32::from_le_bytes(tx[20..24].try_into().unwrap()), tx[DATA_AT..n].to_vec())
}

#[test]
fn unbound_reads_link_down_and_frames_wait() {
    assert_eq!(ask(None, OP_HEALTHCHECK, &[]), (0, vec![]));
    assert_eq!(ask(None, OP_LINK_STATUS, &[]), (0, vec![0]));
    assert_eq!(ask(None, OP_MAC_ADDRESS, &[]).0, E_AGAIN);
    assert_eq!(ask(None, OP_RX_PACKET, &[]).0, E_AGAIN);
    assert_eq!(ask(None, 0x77, &[]).0, E_INVAL);
}

#[test]
fn bound_carries_frames_both_ways() {
    let frame = vec![0xAB; 60];
    let mut nic = Fake { sent: vec![], frames: vec![Ok(Some(frame.clone())), Ok(None)] };
    assert_eq!(ask(Some(&mut nic), OP_LINK_STATUS, &[]), (0, vec![1]));
    assert_eq!(ask(Some(&mut nic), OP_MAC_ADDRESS, &[]), (0, vec![2, 0, 0, 0, 0, 1]));
    let (st, body) = ask(Some(&mut nic), OP_RX_PACKET, &[]);
    assert_eq!((st, &body[..4], &body[4..]), (0, &60u32.to_le_bytes()[..], &frame[..]));
    assert_eq!(ask(Some(&mut nic), OP_RX_PACKET, &[]).0, E_AGAIN);
    assert_eq!(ask(Some(&mut nic), OP_TX_PACKET, &frame).0, 0);
    assert_eq!(nic.sent, vec![frame]);
}

#[test]
fn frames_of_a_wrong_size_never_reach_the_device() {
    let mut nic = Fake { sent: vec![], frames: vec![] };
    assert_eq!(ask(Some(&mut nic), OP_TX_PACKET, &[0; 13]).0, E_INVAL);
    assert_eq!(ask(Some(&mut nic), OP_TX_PACKET, &[0; 1515]).0, E_INVAL);
    let req = Request { op: OP_TX_PACKET, flags: 0, request_id: 1, payload_len: 99 };
    let mut tx = vec![0u8; 64];
    answer(Some(&mut nic), &mut Stats::default(), &req, &[0; 60], &mut tx);
    assert_eq!(i32::from_le_bytes(tx[20..24].try_into().unwrap()), E_MSGSIZE);
    assert!(nic.sent.is_empty());
}
