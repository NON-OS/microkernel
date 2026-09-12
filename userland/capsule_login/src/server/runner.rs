use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_yield};

use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_END_SESSION, OP_GET_STATE,
    OP_HEALTHCHECK, OP_START_SESSION,
};
use crate::server::{handlers, respond};
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK: u64 = 0;
const RECV_NOWAIT: u64 = 1;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        // `drain` returns as soon as a receive fails, and the first receive of
        // each pass asks to block. If that blocking receive ever returns an
        // error instead of waiting, this loop calls it again immediately and
        // the capsule spins a core for as long as the error persists, with
        // nothing on the serial line to say so.
        //
        // Yielding on that path costs one scheduler round trip in the normal
        // case, where the receive blocked and returned a real message, and
        // turns a persistent error from a spin into an idle retry.
        if !drain(&mut ctx, &mut rx, &mut tx) {
            mk_yield();
        }
    }
}

/// Serve everything waiting. Returns whether any request was handled, so the
/// caller can tell a pass that did work from one that fell straight out.
fn drain(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8]) -> bool {
    let mut blocking = true;
    let mut served = false;
    loop {
        let mut sender_pid = 0u32;
        let timeout = if blocking { RECV_BLOCK } else { RECV_NOWAIT };
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            return served;
        }
        served = true;
        blocking = false;
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(v) => v,
            Err((req, errno)) => {
                let _ = respond::status(sender_pid, &req, errno, tx);
                continue;
            }
        };
        match req.op {
            OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
            OP_START_SESSION => handlers::start_session::handle(ctx, sender_pid, &req, body, tx),
            OP_END_SESSION if body.is_empty() => {
                handlers::end_session::handle(ctx, sender_pid, &req, tx)
            }
            OP_GET_STATE if body.is_empty() => {
                handlers::get_state::handle(ctx, sender_pid, &req, tx)
            }
            _ if body.is_empty() => {
                let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
            }
            _ => {
                let _ = respond::status(sender_pid, &req, E_INVAL, tx);
            }
        }
    }
}
