// NONOS Operating System (AGPL-3.0-or-later)
/* The run loop's handling of one request, and a numbered frame builder,
 * shared by the kept-answer tests. */

use crate::kept::{again, forget, keep};
use crate::request::{ask, Ask, STREAM_NUMBERED};

pub fn numbered(seq: u32, payload: &[u8]) -> Vec<u8> {
    let mut f = vec![STREAM_NUMBERED];
    f.extend_from_slice(&seq.to_le_bytes());
    f.extend_from_slice(payload);
    f
}

/// What run.rs does for one request from `pid`, with `drain` standing in
/// for feed(): each call takes whatever the exit has delivered so far.
pub fn serve(pid: u32, frame: &[u8], drain: &mut dyn FnMut() -> Vec<u8>) -> Vec<u8> {
    match ask(frame) {
        Some(Ask::Numbered(seq, _)) => match again(pid, seq) {
            Some(out) => out,
            None => {
                let out = drain();
                keep(pid, seq, &out);
                out
            }
        },
        Some(Ask::Stream(_)) => drain(),
        _ => {
            forget(pid);
            Vec::new()
        }
    }
}
