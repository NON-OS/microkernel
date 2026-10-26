// NONOS Operating System (AGPL-3.0-or-later)
//! A single frame whose reply came too late is not lost: net.core asks the
//! same number again and gets the same frame (nonos_mac::resend).

use crate::fake_nic::Fake;
use crate::nnet::{answer, Request, Stats, DATA_AT, E_AGAIN, OP_RX_PACKET};
use nonos_mac::resend::Kept;

fn rx(nic: &mut Fake, state: (&mut Stats, &mut Kept<1514>), seq: Option<u32>) -> Vec<u8> {
    let body = seq.map(u32::to_le_bytes);
    let body: &[u8] = body.as_ref().map_or(&[], |b| b);
    let req = Request { op: OP_RX_PACKET, flags: 0, request_id: 1, payload_len: body.len() as u32 };
    let mut tx = vec![0u8; 2048];
    let n = answer(Some(nic), state, &req, body, &mut tx);
    tx[DATA_AT..n].to_vec()
}

#[test]
fn a_reask_gets_the_same_frame_and_none_is_lost() {
    let (a, b) = (vec![0xA1; 60], vec![0xB2; 60]);
    let mut nic = Fake { sent: vec![], frames: vec![Ok(Some(a.clone())), Ok(Some(b.clone()))] };
    let (mut stats, mut kept) = (Stats::default(), Kept::new());
    let first = rx(&mut nic, (&mut stats, &mut kept), Some(1));
    assert_eq!(first[4..], a[..]);
    assert_eq!(
        rx(&mut nic, (&mut stats, &mut kept), Some(1))[4..],
        a[..],
        "the late reply's frame again"
    );
    assert_eq!(rx(&mut nic, (&mut stats, &mut kept), Some(2))[4..], b[..]);
    assert!(nic.frames.is_empty());
    assert_eq!(stats.rx_frames, 2, "a resend is not counted again");
}

#[test]
fn the_old_wire_still_takes_a_fresh_frame_each_time() {
    let mut nic = Fake { sent: vec![], frames: vec![Ok(Some(vec![1; 60])), Ok(None)] };
    let (mut stats, mut kept) = (Stats::default(), Kept::new());
    assert_eq!(rx(&mut nic, (&mut stats, &mut kept), None)[4..], [1; 60][..]);
    let req = Request { op: OP_RX_PACKET, flags: 0, request_id: 1, payload_len: 0 };
    let mut tx = vec![0u8; 2048];
    let n = answer(Some(&mut nic), (&mut stats, &mut kept), &req, &[], &mut tx);
    assert_eq!(
        i32::from_le_bytes(tx[20..24].try_into().unwrap()),
        E_AGAIN,
        "nothing kept for the old wire"
    );
    assert!(n > 0);
}
