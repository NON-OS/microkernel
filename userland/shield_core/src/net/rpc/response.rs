//! Reading a JSON-RPC reply from a machine the wallet does not control. A strict reader for one
//! shape, refusing an `error` member, bad hex, or a topic that is not 32 bytes.

use crate::error::NetError;

#[path = "response_hex.rs"]
mod hex;
#[path = "response_logs.rs"]
mod log;
#[path = "response_scan.rs"]
mod scan;

pub(crate) use hex::hex_bytes;
use hex::u64_hex;
pub use log::{logs, RawLog};
pub(crate) use scan::{after, objects, quoted};

/// The block number an `eth_blockNumber` reply carries.
pub fn block_number(reply: &str) -> Result<u64, NetError> {
    refuse_error(reply)?;
    let result = after(reply, "\"result\"").ok_or(NetError::ReplyShape)?;
    let hex = quoted(result).ok_or(NetError::ReplyShape)?;
    u64_hex(hex).ok_or(NetError::ReplyShape)
}

/// The bytes an `eth_call` returned.
pub fn call_result(reply: &str) -> Result<Vec<u8>, NetError> {
    refuse_error(reply)?;
    let result = after(reply, "\"result\"").ok_or(NetError::ReplyShape)?;
    hex_bytes(quoted(result).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)
}

/// A batch of `n` results placed by id. An error, a missing id or a repeat refuses the batch.
/// `n` is the caller's own count of the calls it sent, never a peer's number.
pub fn call_results(reply: &str, n: usize) -> Result<Vec<Vec<u8>>, NetError> {
    refuse_error(reply)?;
    let mut out: Vec<Option<Vec<u8>>> = vec![None; n];
    for object in objects(reply) {
        let id_at = after(object, "\"id\"").ok_or(NetError::ReplyShape)?;
        let id_text: String = id_at
            .trim_start_matches(|c: char| c == ':' || c.is_whitespace())
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        let id: usize = id_text.parse().map_err(|_| NetError::ReplyShape)?;
        let result = after(object, "\"result\"").ok_or(NetError::ReplyShape)?;
        let bytes =
            hex_bytes(quoted(result).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)?;
        let slot = out.get_mut(id).ok_or(NetError::ReplyShape)?;
        if slot.replace(bytes).is_some() {
            return Err(NetError::ReplyShape);
        }
    }
    out.into_iter().map(|r| r.ok_or(NetError::ReplyShape)).collect()
}

/// Refuse a reply carrying a JSON-RPC error.
fn refuse_error(reply: &str) -> Result<(), NetError> {
    if reply.contains("\"error\"") {
        return Err(NetError::ReplyShape);
    }
    Ok(())
}
