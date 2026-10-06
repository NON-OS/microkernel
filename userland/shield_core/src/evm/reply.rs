//! Reading an account RPC's reply strictly, one expected shape per call. A batch
//! is split by id, and a reverted call keeps its revert data.

use crate::error::NetError;
use crate::net::rpc::{after, hex_bytes, objects, quoted};

/// One call's answer: its result as raw JSON text, or what the node refused.
pub(super) enum Answer {
    Result(String),
    /// The call reverted, with the revert data when the node returned it.
    Reverted(Vec<u8>),
    /// The node refused the call for another reason. Reading it as a value fails.
    Failed,
}

/// The answers of a batch of `n` calls, placed by id. A bad id refuses them all.
pub(super) fn batch(reply: &str, n: usize) -> Result<Vec<Answer>, NetError> {
    let mut out: Vec<Option<Answer>> = (0..n).map(|_| None).collect();
    for object in objects(reply) {
        let id = id_of(object).ok_or(NetError::ReplyShape)?;
        let slot = out.get_mut(id).ok_or(NetError::ReplyShape)?;
        if slot.replace(one(object)?).is_some() {
            return Err(NetError::ReplyShape);
        }
    }
    out.into_iter().map(|a| a.ok_or(NetError::ReplyShape)).collect()
}

fn id_of(object: &str) -> Option<usize> {
    let at = after(object, "\"id\"")?;
    let digits: String = at
        .trim_start_matches(|c: char| c == ':' || c.is_whitespace())
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}

/// One reply object. An error with revert data or an execution revert is a revert.
pub(super) fn one(object: &str) -> Result<Answer, NetError> {
    if let Some(error) = after(object, "\"error\"") {
        let data = after(error, "\"data\"").and_then(quoted).and_then(hex_bytes);
        let reverted = error.contains("revert") || data.is_some();
        return Ok(if reverted {
            Answer::Reverted(data.unwrap_or_default())
        } else {
            Answer::Failed
        });
    }
    let result = after(object, "\"result\"").ok_or(NetError::ReplyShape)?;
    let text = result.trim_start_matches(|c: char| c == ':' || c.is_whitespace());
    Ok(Answer::Result(text.to_string()))
}

#[cfg(test)]
#[path = "reply_test.rs"]
mod reply_test;

#[cfg(kani)]
#[path = "reply_kani.rs"]
mod reply_kani;
