// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The service loop: one request at a time from the wallet window, one
//! reply each. Answers are immediate; long work goes to the worker.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_service_lookup};
use shield_wire::*;

use crate::{jobs, ops};

const MAX_MSG: usize = MESSAGE_MAX;

/// The wallet window's service names; nothing else may call the shield.
const WALLETS: [&[u8]; 3] = [b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

fn from_wallet(sender: u32) -> bool {
    WALLETS.iter().any(|name| {
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        rc >= 0 && pid != 0 && pid == sender
    })
}

fn answer(status: i32, values: &Values) -> (i32, String) {
    (status, String::from(values.text()))
}

fn refused(why: &str) -> (i32, String) {
    let mut v = Values::new();
    v.put("why", why);
    (STATUS_REFUSED, String::from(v.text()))
}

fn now(result: ops::Answer) -> (i32, String) {
    match result {
        Ok(v) => answer(STATUS_OK, &v),
        Err(why) => refused(&why),
    }
}

/// Hand `work` to the worker under `name`.
fn job(name: &'static str, work: impl FnOnce() -> ops::Answer + Send + 'static) -> (i32, String) {
    if jobs::start(name, work) {
        (STATUS_STARTED, String::new())
    } else {
        let mut v = Values::new();
        v.put("job", jobs::running().unwrap_or("another"));
        (STATUS_BUSY, String::from(v.text()))
    }
}

fn owned(fields: &[&str]) -> Vec<String> {
    fields.iter().map(|s| String::from(*s)).collect()
}

fn borrowed(fields: &[String]) -> Vec<&str> {
    fields.iter().map(String::as_str).collect()
}

/* Recovery words and keys: zeroed when dropped, whether the job ran or was refused. */
struct Secret(Vec<String>);

impl Drop for Secret {
    fn drop(&mut self) {
        for f in self.0.iter_mut() {
            // SAFETY: zero bytes are valid UTF-8, and the string is dropped right after.
            for b in unsafe { f.as_bytes_mut() } {
                unsafe { core::ptr::write_volatile(b, 0) };
            }
        }
    }
}

fn opening(
    fields: Vec<String>,
    open: fn(&[&str]) -> ops::Answer,
) -> impl FnOnce() -> ops::Answer + Send + 'static {
    let secret = Secret(fields);
    move || open(&borrowed(&secret.0))
}

fn dispatch(op: u16, body: &str) -> (i32, String) {
    let fields: Vec<&str> = if body.is_empty() { Vec::new() } else { body.split('\n').collect() };
    let f = owned(&fields);
    match op {
        OP_STATE => now(ops::state()),
        OP_OPEN_WORDS => job("open", opening(f, ops::open_words)),
        OP_OPEN_KEY => job("open", opening(f, ops::open_key)),
        OP_LOCK => now(ops::lock()),
        OP_RESULT => answer(STATUS_OK, &jobs::result()),
        OP_CANCEL => now(ops::cancel()),
        OP_FRESH_ADDRESS => now(ops::fresh_address()),
        OP_SYNC => job("sync", ops::sync),
        OP_REVIEW_SHIELD => job("review shield", move || ops::review_shield(&borrowed(&f))),
        OP_CONFIRM => job("confirm", move || ops::confirm(&borrowed(&f))),
        OP_QUOTE => job("quote", move || ops::quote(&borrowed(&f))),
        OP_SEND => job("send", move || ops::send(&borrowed(&f))),
        OP_WITHDRAW => job("withdraw", move || ops::withdraw(&borrowed(&f))),
        OP_FOLLOW => job("follow", ops::follow),
        OP_REVIEW_SELF_SETTLE => {
            job("review settle", move || ops::review_self_settle(&borrowed(&f)))
        }
        OP_TAKE_BACK => job("take back", ops::take_back),
        _ => (STATUS_MALFORMED, String::new()),
    }
}

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    loop {
        let mut sender: u32 = 0;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut sender);
        if n <= 0 {
            continue;
        }
        let used = n as usize;
        let reply = match decode_request(&buf[..used]) {
            None => encode_reply(0, STATUS_MALFORMED, ""),
            Some(req) if !from_wallet(sender) => encode_reply(req.seq, STATUS_DENIED, ""),
            Some(req) => {
                let (status, body) = dispatch(req.op, req.body);
                encode_reply(req.seq, status, &body)
            }
        };
        // The answer goes to the window that called, matched to its pending call by the
        // kernel. It was sent to a fixed reply endpoint, 0x1_0000_0002, which is the
        // keyring's and not this service's own reply inbox: the kernel routed every answer
        // there as an ordinary send, so the wallet's call always ran out of time and the
        // shield read as still starting. Only a wallet window calls, never the kernel.
        if sender != 0 {
            let _ = mk_ipc_reply(sender, reply.as_ptr(), reply.len());
        }
        // A request can carry recovery words: nothing of it stays.
        for b in buf[..used].iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
    }
}
