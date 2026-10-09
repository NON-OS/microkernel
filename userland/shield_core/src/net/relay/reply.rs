//! The lander's replies, read strictly. Text it sends that a screen will
//! show, a refusal reason, is kept to printable characters and 200 of them:
//! the lander is a server the wallet does not control.

use crate::error::NetError;
use crate::net::rpc::{after, quoted};

/// What became of a hand-off the moment it was posted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Handed {
    Queued { id: String },
    Refused { reason: String },
}

/// Where a hand-off stands, as the lander reports it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RelayState {
    /// One of queued, scheduled, settling, settled or refused.
    pub status: String,
    /// The settlement, once there is one.
    pub tx: Option<String>,
    pub reason: Option<String>,
}

fn field<'a>(body: &'a str, key: &str) -> Option<&'a str> {
    after(body, &format!("\"{key}\"")).and_then(quoted)
}

fn printable(text: &str) -> String {
    text.chars().filter(|c| c.is_ascii_graphic() || *c == ' ').take(200).collect()
}

pub(crate) fn handed(status: u16, body: &str) -> Result<Handed, NetError> {
    match status {
        202 => {
            let id = field(body, "id").ok_or(NetError::ReplyShape)?;
            let fine = !id.is_empty()
                && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
            if fine {
                Ok(Handed::Queued { id: id.to_string() })
            } else {
                Err(NetError::ReplyShape)
            }
        }
        400 => {
            Ok(Handed::Refused { reason: printable(field(body, "reason").unwrap_or("refused")) })
        }
        other => Err(NetError::Rejected { code: other }),
    }
}

pub(crate) fn state(body: &str) -> Result<RelayState, NetError> {
    let status = field(body, "status").ok_or(NetError::ReplyShape)?;
    if !["queued", "scheduled", "settling", "settled", "refused"].contains(&status) {
        return Err(NetError::ReplyShape);
    }
    let tx = field(body, "tx").filter(|t| {
        t.len() == 66 && t.starts_with("0x") && t.bytes().skip(2).all(|b| b.is_ascii_hexdigit())
    });
    Ok(RelayState {
        status: status.to_string(),
        tx: tx.map(str::to_string),
        reason: field(body, "reason").map(printable),
    })
}

/// Whether `/v1/info` names `pool` as the pool the lander serves.
pub(crate) fn serves(body: &str, pool: &str) -> bool {
    field(body, "pool").is_some_and(|p| p.eq_ignore_ascii_case(pool))
}

/// Whether `/v1/info` says the lander takes a proof whose fee goes to whoever submits it.
pub(crate) fn takes_submitter_fee(body: &str) -> bool {
    after(body, "\"fee_to_submitter_accepted\"")
        .map(|rest| rest.trim_start_matches([' ', ':']))
        .is_some_and(|rest| rest.starts_with("true"))
}

#[cfg(test)]
#[path = "reply_test.rs"]
mod reply_test;
