// NONOS Operating System (AGPL-3.0-or-later)
//! driver.xhci0's replies with hostile bytes and lengths, through the
//! driver's own decoders (xhci/reply.rs, xhci/port_list.rs): nothing panics,
//! and nothing is read past what came.

use crate::xhci::port_list::connected;
use crate::xhci::reply::{data_in, moved, reply};
use crate::xhci::wire::{DATA_AT, HDR_LEN, MAGIC};
use crate::xhci::{Port, PORT_FREE};

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn a_port_list_keeps_the_connected_ports_its_count_and_length_allow() {
    let mut resp = vec![0u8; DATA_AT + 4 + 3 * 8];
    resp[DATA_AT] = 3;
    for (i, (id, portsc)) in [(1u8, 1u32), (2, 0), (3, 0x201)].into_iter().enumerate() {
        let at = DATA_AT + 4 + i * 8;
        (resp[at], resp[at + 1]) = (id, PORT_FREE);
        resp[at + 4..at + 8].copy_from_slice(&portsc.to_le_bytes());
    }
    let ids = |r: Result<Vec<Port>, i32>| r.map(|p| p.iter().map(|p| p.id).collect::<Vec<u8>>());
    assert_eq!(ids(connected(&resp, 4 + 24)), Ok(vec![1, 3]));
    assert!(connected(&resp, 4 + 23).is_err(), "a count past the reply");
    resp[DATA_AT] = 255;
    assert!(connected(&resp, 4 + 24).is_err());
    assert!(connected(&resp, usize::MAX).is_err(), "a count past the buffer");
}

#[test]
fn hostile_replies_never_panic_and_never_reach_past_what_came() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..200_000 {
        let len = (xorshift(&mut seed) % 120) as usize;
        let mut resp: Vec<u8> = (0..len).map(|_| xorshift(&mut seed) as u8).collect();
        if len >= DATA_AT && seed & 3 != 0 {
            resp[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            resp[12..16].copy_from_slice(&7u32.to_le_bytes());
            resp[16..20].copy_from_slice(&((xorshift(&mut seed) % 128) as u32).to_le_bytes());
            resp[HDR_LEN..DATA_AT].fill(0);
        }
        let n = (xorshift(&mut seed) % 160) as i64 - 16;
        if let Ok(data) = reply(&resp, n, 7) {
            assert!(DATA_AT + data <= n as usize && n as usize <= resp.len());
        }
        let (rlen, head) = ((xorshift(&mut seed) % 128) as usize, 2 + 2 * (seed & 1) as usize);
        let mut out = vec![0u8; (xorshift(&mut seed) % 40) as usize];
        if let Ok(count) = data_in(&resp, rlen, head, &mut out) {
            assert!(count <= out.len() && head + count <= rlen && DATA_AT + head + count <= len);
        }
        if let Ok(ports) = connected(&resp, rlen) {
            assert!(4 + 8 * ports.len() <= rlen);
        }
        assert!(moved(&resp, rlen).is_err() || rlen >= 4);
    }
}
