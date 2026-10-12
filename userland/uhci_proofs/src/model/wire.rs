// NONOS Operating System (AGPL-3.0-or-later)
//! A request as a class driver sends it, through the shipped dispatch.

use super::handles::Cx;
use crate::server::dispatch::dispatch;
use crate::server::many::{serve, Front};
use crate::wire::{decode, Request, HDR_LEN, REPLY_MAX, STATUS_LEN};

/// The header's magic and version, as every class driver writes them.
const MAGIC: u32 = 0x4E58_4843;
const VERSION: u16 = 1;

#[derive(Debug)]
pub struct Answer {
    pub status: i32,
    pub data: Vec<u8>,
    pub request_id: u32,
}

pub fn call(cx: &mut Cx, op: u16, body: &[u8]) -> Answer {
    call_with(op, body, |req, body, tx| dispatch(cx, req, body, tx))
}

/// A request to the host front over several controllers (server/many).
pub fn call_host(f: &mut Front<Cx>, op: u16, body: &[u8]) -> Answer {
    call_with(op, body, |req, body, tx| serve(f, req, body, tx))
}

fn call_with(
    op: u16,
    body: &[u8],
    serve: impl FnOnce(&Request, &[u8], &mut [u8]) -> usize,
) -> Answer {
    let mut rx = vec![0u8; HDR_LEN];
    rx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    rx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    rx[6..8].copy_from_slice(&op.to_le_bytes());
    rx[12..16].copy_from_slice(&0x5A5Au32.to_le_bytes());
    rx[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    let req = decode(&rx).expect("a well-formed header");
    let mut tx = vec![0u8; HDR_LEN + STATUS_LEN + REPLY_MAX];
    let n = serve(&req, body, &mut tx);
    let len = u32::from_le_bytes(tx[16..20].try_into().unwrap()) as usize;
    assert_eq!(n, HDR_LEN + len, "the reply length matches its header");
    assert_eq!(u16::from_le_bytes([tx[6], tx[7]]), op, "the op is echoed");
    Answer {
        status: i32::from_le_bytes(tx[20..24].try_into().unwrap()),
        data: tx[HDR_LEN + STATUS_LEN..n].to_vec(),
        request_id: u32::from_le_bytes(tx[12..16].try_into().unwrap()),
    }
}
