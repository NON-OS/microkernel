//! The pool state agreed by every RPC that answers.

use super::super::deposit_gate::PoolState;
use super::state::read_pool_state;
use crate::error::NetError;
use crate::net::pool::Pool;
use crate::net::tor::Tor;

/// The pool state from every RPC in `hosts` that answers. Two answers must agree, since a lone
/// RPC could report a wound-down pool as open and trap a deposit. Disagreement refuses the read.
pub fn read_pool_state_checked(
    tor: &Tor,
    hosts: &[&str],
    pool: &Pool,
    depositor: &[u8; 20],
    asset: u64,
) -> Result<PoolState, NetError> {
    let mut agreed: Option<PoolState> = None;
    let mut last = NetError::Transport;
    for host in hosts {
        match read_pool_state(tor, host, pool, depositor, asset) {
            Ok(state) => match agreed {
                None => agreed = Some(state),
                Some(prior) if prior == state => {}
                Some(_) => return Err(NetError::ReplyShape),
            },
            Err(e) => last = e,
        }
    }
    agreed.ok_or(last)
}
