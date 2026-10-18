// NONOS Operating System (AGPL-3.0-or-later)
//! IPC: driver.xhci0 played by the proof. Every call is recorded with the
//! timeout it was given; the responder returns the whole reply, cut to
//! the caller's buffer as the kernel would, or `None` for a call that got
//! no answer in time.

use std::cell::RefCell;

type Responder = Box<dyn FnMut(&[u8]) -> Option<Vec<u8>>>;

/// The port driver.xhci0 is found at while a proof plays it.
const XHCI_PORT: u32 = 0x4011;
const E_NOENT: i64 = -2;
const E_TIMEDOUT: i64 = -110;

/// One call the core made: the request bytes and its timeout.
#[derive(Clone, Debug)]
pub struct Asked {
    pub req: Vec<u8>,
    pub timeout_ms: u64,
}

thread_local! {
    static RESPONDER: RefCell<Option<Responder>> = const { RefCell::new(None) };
    static ASKED: RefCell<Vec<Asked>> = const { RefCell::new(Vec::new()) };
}

/// Play driver.xhci0 on this thread with `f`.
pub fn play(f: impl FnMut(&[u8]) -> Option<Vec<u8>> + 'static) {
    RESPONDER.with(|r| *r.borrow_mut() = Some(Box::new(f)));
}

/// Every call made on this thread so far, oldest first.
pub fn calls() -> Vec<Asked> {
    ASKED.with(|a| a.borrow().clone())
}

pub fn mk_ipc_call_timeout(
    _port: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    cap: usize,
    timeout_ms: u64,
) -> i64 {
    // SAFETY: the core passes pointers to its own buffers with their lengths.
    let req = unsafe { core::slice::from_raw_parts(req, req_len) }.to_vec();
    ASKED.with(|a| a.borrow_mut().push(Asked { req: req.clone(), timeout_ms }));
    let reply = RESPONDER.with(|r| r.borrow_mut().as_mut().map(|f| f(&req)));
    let Some(Some(reply)) = reply else { return E_TIMEDOUT };
    let n = reply.len().min(cap);
    // SAFETY: `resp` holds `cap` bytes and `n <= cap`.
    unsafe { core::ptr::copy_nonoverlapping(reply.as_ptr(), resp, n) };
    n as i64
}

/// Only driver.xhci0 registers: the played host is the one host there is,
/// so a search over every host name reaches it once.
pub fn mk_service_lookup(name: *const u8, len: usize, port: *mut u32, _pid: *mut u32) -> i64 {
    // SAFETY: the core passes its own name bytes with their length.
    let name = unsafe { core::slice::from_raw_parts(name, len) };
    if RESPONDER.with(|r| r.borrow().is_none()) || port.is_null() || name != b"driver.xhci0" {
        return E_NOENT;
    }
    // SAFETY: the core passes a pointer to its own u32.
    unsafe { *port = XHCI_PORT };
    0
}
