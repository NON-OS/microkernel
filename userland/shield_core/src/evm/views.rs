//! View calls for the modules beside this one: a batch of reads of one contract, made as `from`
//! when one is given, on the circuits of the network, after its chain id. A revert keeps its data,
//! so a refusal can be named.

use super::calls::call;
use super::network::Network;
use super::reply::Answer;
use super::rpc::{ask, Call};
use crate::error::NetError;
use crate::net::tor::Tor;

/// What one view call gave back.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Viewed {
    Data(Vec<u8>),
    Reverted(Vec<u8>),
}

pub fn view(
    tor: &Tor,
    network: Network,
    from: Option<&[u8; 20]>,
    to: &[u8; 20],
    datas: &[Vec<u8>],
) -> Result<Vec<Viewed>, NetError> {
    let calls: Vec<Call> = datas.iter().map(|d| call(from, to, d)).collect();
    let answers = ask(tor, &network.chain(), &calls)?;
    answers
        .into_iter()
        .map(|answer| match answer {
            Answer::Reverted(data) => Ok(Viewed::Reverted(data)),
            Answer::Failed => Err(NetError::ReplyShape),
            done @ Answer::Result(_) => Ok(Viewed::Data(done.data()?)),
        })
        .collect()
}
