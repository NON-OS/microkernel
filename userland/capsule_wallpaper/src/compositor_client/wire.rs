use alloc::vec::Vec;

mod payload;
mod reply;
mod status;

pub(crate) use payload::call_payload_boot;
pub(crate) use status::{call, call_boot, call_within};

pub(super) const NCMP_MAGIC: u32 = 0x4E43_4D50;
pub(super) const NCMP_VERSION: u16 = 1;
pub(super) const NCMP_HDR_LEN: usize = 20;
pub(super) const CALL_REPLY_TIMEOUT_MS: u64 = 16;
pub(super) const BOOT_REPLY_TIMEOUT_MS: u64 = 250;

/// The call reached the compositor and its answer did not come in time. The
/// compositor reads its requests between frames and a full frame can take
/// longer than a call's budget, so this is mostly a busy compositor, not a
/// lost request: the kernel queued the request when the call was sent, the
/// compositor still acts on it, and only its late answer is dropped.
pub(crate) const TIMED_OUT: &str = "compositor call timed out";
/// The kernel's ETIMEDOUT, what an unanswered call returns.
const ETIMEDOUT: i64 = -110;

pub(super) fn build_request(out: &mut Vec<u8>, op: u16, request_id: u32, payload: &[u8]) {
    out.clear();
    out.extend_from_slice(&NCMP_MAGIC.to_le_bytes());
    out.extend_from_slice(&NCMP_VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}
