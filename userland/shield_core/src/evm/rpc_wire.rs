//! One batch on the wire: its JSON body, and one RPC's answer checked for its chain.

use super::super::network::Chain;
use super::super::reply::{batch, Answer};
use super::Call;
use crate::error::NetError;
use crate::net::rpc::post;
use crate::net::tor::{tls, Tor};

pub(super) fn once(
    tor: &Tor,
    chain: &Chain,
    host: &str,
    body: &str,
    n: usize,
) -> Result<Vec<Answer>, NetError> {
    let stream = tls(tor.connect(chain.purpose, host, 443)?, host)?;
    let reply = post(stream, host, "/", body)?;
    let text = core::str::from_utf8(&reply).map_err(|_| NetError::ReplyShape)?;
    let mut answers = batch(text, n.checked_add(1).ok_or(NetError::ReplyShape)?)?.into_iter();
    let id = answers.next().ok_or(NetError::ReplyShape)?.quantity()?;
    if id != u128::from(chain.chain_id) {
        return Err(NetError::WrongChain);
    }
    Ok(answers.collect())
}

pub(super) fn body(calls: &[Call]) -> String {
    let mut items = vec![r#"{"jsonrpc":"2.0","id":0,"method":"eth_chainId","params":[]}"#.into()];
    for (i, call) in calls.iter().enumerate() {
        let id = i.saturating_add(1);
        items.push(format!(
            r#"{{"jsonrpc":"2.0","id":{id},"method":"{}","params":{}}}"#,
            call.method, call.params
        ));
    }
    format!("[{}]", items.join(","))
}
