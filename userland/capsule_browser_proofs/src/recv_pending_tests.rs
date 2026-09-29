// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
/* A read that asks for more than its caller can hold keeps the rest for the
 * next read on that socket, so callers with small buffers still see every
 * byte, in order, once. */

use crate::browser::recv_pending::{forget, put, take};

#[test]
fn small_readers_drain_one_large_answer_in_order() {
    let answer: Vec<u8> = (0..40_000u32).map(|i| (i % 251) as u8).collect();
    let (handle, mut first) = (9001, [0u8; 64]);
    let copied = first.len();
    first.copy_from_slice(&answer[..copied]);
    put(handle, &answer[copied..]);
    let mut seen = first.to_vec();
    let mut buf = [0u8; 4096];
    loop {
        let n = take(handle, &mut buf);
        if n == 0 {
            break;
        }
        seen.extend_from_slice(&buf[..n]);
    }
    assert_eq!(seen, answer, "every byte once, in order");
}

#[test]
fn sockets_keep_their_own_leftovers() {
    put(9101, b"alpha");
    put(9102, b"beta");
    put(9101, b"-more");
    let mut buf = [0u8; 16];
    let n = take(9102, &mut buf);
    assert_eq!(&buf[..n], b"beta");
    let n = take(9101, &mut buf);
    assert_eq!(&buf[..n], b"alpha-more", "appended after what was held");
    assert_eq!(take(9101, &mut buf), 0);
}

#[test]
fn a_closed_socket_drops_what_it_held() {
    put(9201, b"stale");
    forget(9201);
    let mut buf = [0u8; 8];
    assert_eq!(take(9201, &mut buf), 0, "a reused handle starts empty");
}
