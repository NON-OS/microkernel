//! The fast route: chunks straight from the mirror's address, off the
//! mixnet, several at once on sockets of this capsule's own. Built only
//! with `install-fast-route`, taken only when the person turned it on
//! (route_pick.rs). The mirror learns this machine's address and that it
//! fetched this image; what it serves is held to the same digests as on
//! the mixnet, by the same code (run.rs).

use alloc::vec::Vec;
use core::ops::Range;

use nonos_libc::{mk_yield, Deadline};

use super::super::http::get::request;
use super::super::http_reply::{body, complete};
use super::super::timing;
use super::fetch::Mirror;
use super::span;
use crate::linux::net::raw_io::{close, recv_until, send_all};
use crate::linux::net::sockets::call::call;
use crate::linux::net::sockets::ops::{DOMAIN, KIND_STREAM, OP_CONNECT_HOST, OP_RECV, OP_SOCKET};

/// How long the mirror may go silent on one reply.
const IDLE_MS: u64 = 60_000;
/// The longest list (cache.rs LIST_MAX) and a reply's head beside it.
const LIST_REPLY: usize = (32 + 32 * 16384) + (16 << 10);
const HEAD: usize = 16 << 10;
/// One read's most.
const READ: usize = 32 << 10;

pub fn list(m: &Mirror, path: &str) -> Option<Vec<u8>> {
    let h = open(m)?;
    let t0 = timing::now();
    let raw = send_all(h, request(path, m.host).as_bytes())
        .and_then(|()| recv_until(h, LIST_REPLY, &complete, IDLE_MS));
    close(h);
    timing::got(raw.as_ref().map_or(0, Vec::len), timing::now().saturating_sub(t0));
    raw.and_then(body)
}

/// `ranges` asked at once, one socket each; each read as it comes.
pub fn many(m: &Mirror, path: &str, ranges: &[Range<u64>], total: u64) -> Vec<Option<Vec<u8>>> {
    let t0 = timing::now();
    let open: Vec<Option<u32>> = ranges
        .iter()
        .map(|r| {
            let h = open(m)?;
            match send_all(h, span::request(path, m.host, r).as_bytes()) {
                Some(()) => Some(h),
                None => {
                    close(h);
                    None
                }
            }
        })
        .collect();
    let mut got: Vec<Vec<u8>> = ranges.iter().map(|_| Vec::new()).collect();
    let mut done: Vec<bool> = open.iter().map(Option::is_none).collect();
    let mut quiet = Deadline::after_ms(IDLE_MS);
    while !done.iter().all(|d| *d) && !quiet.expired() {
        let mut moved = false;
        for (i, h) in open.iter().enumerate() {
            let (Some(h), false) = (h, done[i]) else { continue };
            let cap = usize::try_from(ranges[i].end - ranges[i].start).unwrap_or(usize::MAX);
            if let Some((0, part)) = call(OP_RECV, &h.to_le_bytes(), READ) {
                moved |= !part.is_empty();
                got[i].extend_from_slice(&part);
            }
            done[i] = complete(&got[i]) || got[i].len() > cap.saturating_add(HEAD);
        }
        match moved {
            true => quiet = Deadline::after_ms(IDLE_MS),
            false => drop(mk_yield()),
        }
    }
    open.iter().flatten().for_each(|h| close(*h));
    let bytes = got.iter().map(Vec::len).sum();
    timing::got(bytes, timing::now().saturating_sub(t0));
    got.into_iter().zip(ranges).map(|(raw, r)| span::chunk(raw, r, total)).collect()
}

/// A stream socket to the mirror's address, dialled directly.
fn open(m: &Mirror) -> Option<u32> {
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&DOMAIN.to_le_bytes());
    body.extend_from_slice(&KIND_STREAM.to_le_bytes());
    let h = match call(OP_SOCKET, &body, 8) {
        Some((0, out)) if out.len() >= 4 => u32::from_le_bytes([out[0], out[1], out[2], out[3]]),
        _ => return None,
    };
    let dial = crate::linux::net::sockets::host_body::host_body(h, m.port, m.at.as_bytes());
    match dial.and_then(|b| call(OP_CONNECT_HOST, &b, 0)) {
        Some((0, _)) => Some(h),
        _ => {
            close(h);
            None
        }
    }
}
