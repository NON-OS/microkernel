//! The logs of an `eth_getLogs` reply, each read strictly into its topics, data and block.

use super::hex::{bytes32, hex_bytes, u64_hex};
use super::refuse_error;
use super::scan::{after, objects, quoted, strings_in_array};
use crate::error::NetError;

/// One log: indexed fields as topics, the rest as data, and the block it landed in.
#[derive(Clone)]
pub struct RawLog {
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub block: u64,
    /// The transaction that emitted it, when the server names one.
    pub tx: Option<[u8; 32]>,
}

/// The `topics` and `data` of each log in an `eth_getLogs` reply.
pub fn logs(reply: &str) -> Result<Vec<RawLog>, NetError> {
    refuse_error(reply)?;
    let array = after(reply, "\"result\"").ok_or(NetError::ReplyShape)?;
    let mut out = Vec::new();
    for object in objects(array) {
        out.push(one_log(object)?);
    }
    Ok(out)
}

fn one_log(object: &str) -> Result<RawLog, NetError> {
    let topics_at = after(object, "\"topics\"").ok_or(NetError::ReplyShape)?;
    let mut topics = Vec::new();
    for hex in strings_in_array(topics_at).ok_or(NetError::ReplyShape)? {
        topics.push(bytes32(hex).ok_or(NetError::ReplyShape)?);
    }
    let data_at = after(object, "\"data\"").ok_or(NetError::ReplyShape)?;
    let data =
        hex_bytes(quoted(data_at).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)?;
    let block_at = after(object, "\"blockNumber\"").ok_or(NetError::ReplyShape)?;
    let block =
        u64_hex(quoted(block_at).ok_or(NetError::ReplyShape)?).ok_or(NetError::ReplyShape)?;
    let tx = after(object, "\"transactionHash\"").and_then(quoted).and_then(bytes32);
    Ok(RawLog { topics, data, block, tx })
}
