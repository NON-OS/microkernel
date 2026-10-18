// NONOS Operating System (AGPL-3.0-or-later)
//! The server's loop on the test clock, as server/runner.rs runs it: a
//! request served, or a receive that times out after `wait_ms`, and one step
//! after either.

use crate::clock::{advance, at};
use crate::fixture::{frame, one_sector};
use crate::protocol::OP_BLK_READ;
use crate::scan::Medium;
use crate::server::respond::{sent, Reply};
use crate::server::serve::Server;

/// The longest a step may take: its window (server/serve.rs STEP_MS), a
/// reset after a failed command (disk/recover.rs RECOVERY_MS), and a slot
/// given back (xhci/slot.rs RELEASE_MS).
pub const STEP_BOUND_MS: u64 = 6_000 + 8_000 + 6_000;

/// The one reply to a request of `op` with `body` from `sender`, and no step
/// yet: for a test that changes the device before the step that follows.
pub fn answer(server: &mut Server, sender: u32, op: u16, body: &[u8]) -> Reply {
    let mut tx = vec![0u8; 64 * 1024];
    let _ = sent();
    server.serve(sender, &frame(op, 7, body), &mut tx);
    let replies = sent();
    assert_eq!(replies.len(), 1, "one reply a request: {replies:?}");
    replies[0]
}

/// A request's reply, then the step the server takes after every request:
/// a device that failed one is checked there, before the next is served.
pub fn ask(server: &mut Server, sender: u32, op: u16, body: &[u8]) -> Reply {
    let reply = answer(server, sender, op, body);
    step(server);
    reply
}

/// The kernel's read of one sector: its status.
pub fn read(server: &mut Server) -> i32 {
    ask(server, 0, OP_BLK_READ, &one_sector(7)).status
}

/// No request before the receive times out, then one step. Returns how long
/// the step took.
pub fn idle(server: &mut Server) -> u64 {
    advance(server.scanner.wait_ms());
    step(server)
}

/// One step, held to its bound.
fn step(server: &mut Server) -> u64 {
    let t0 = at();
    server.step();
    let took = at() - t0;
    assert!(took <= STEP_BOUND_MS, "a step held the server {took} ms");
    took
}

/// Step until the medium is ready, or fail after `steps`.
pub fn until_ready(server: &mut Server, steps: usize) {
    for _ in 0..steps {
        if matches!(server.scanner.medium, Medium::Ready(_)) {
            return;
        }
        idle(server);
    }
    panic!("not bound in {steps} steps: {:?}", server.scanner.report);
}
