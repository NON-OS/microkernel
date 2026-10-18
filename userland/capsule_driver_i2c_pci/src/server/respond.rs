use crate::protocol::{response, wipe, Request};
use nonos_libc::mk_ipc_reply;

/// Build the reply in `out` and send it, then wipe it there: the kernel has
/// copied it by the time mk_ipc_reply returns, and `out` is the server's one
/// reply buffer, reused for every reply, so an input report would otherwise
/// stay in it after it is delivered (plan/50 C8).
pub fn send(sender_pid: u32, req: &Request, errno: i32, body: &[u8], out: &mut [u8]) -> i64 {
    let n = response(req.op, req.request_id, errno, body, out);
    let sent = mk_ipc_reply(sender_pid, out.as_ptr(), n);
    wipe(out.get_mut(..n).unwrap_or_default());
    sent
}
