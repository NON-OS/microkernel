use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::{build_request, reply::decode_status_payload, BOOT_REPLY_TIMEOUT_MS, NCMP_HDR_LEN};

/// A display control answers through the display driver, one monitor
/// reading (at most 200 ms in the compositor) per display: it may take
/// longer than the boot wait, and still answers before Settings' own 1 s.
pub(crate) const CONTROL_REPLY_TIMEOUT_MS: u64 = 800;

pub(crate) fn call_payload_boot(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    body: &mut [u8],
) -> Result<i32, &'static str> {
    call_payload_within(compositor_port, op, request_id, payload, body, BOOT_REPLY_TIMEOUT_MS)
}

pub(crate) fn call_payload_within(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    body: &mut [u8],
    timeout_ms: u64,
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(NCMP_HDR_LEN + payload.len());
    build_request(&mut tx, op, request_id, payload);
    let mut rx = alloc::vec![0u8; NCMP_HDR_LEN + 4 + body.len()];
    let rc = mk_ipc_call_timeout(
        compositor_port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    decode_status_payload(&rx, op, request_id, body, rc)
}
