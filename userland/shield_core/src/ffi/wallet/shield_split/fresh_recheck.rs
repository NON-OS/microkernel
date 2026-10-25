//! ETH deposits asked about again at confirm, before the key is derived: a gas rise the ETH there
//! cannot pay refuses with the review's own line, and one it can pay is reviewed again. The held
//! deposits go out as reviewed or not at all.

use super::confirm::ShieldSplitSent;
use super::fresh_review::GAS_EATS;
use crate::error::WalletError;
use crate::evm::shield_recheck::{recheck_batch, Recheck};
use crate::evm::tx::Eip1559;
use crate::net::tor::Tor;

const FEES_MOVED: &str = "The network fees moved since the review. Review it again.";

/// None when `txs` from `from` still fit the chain; else confirm's answer, with nothing sent.
pub(super) fn eth_refusal(
    tor: &Tor,
    from: &[u8; 20],
    txs: &[Eip1559],
) -> Result<Option<ShieldSplitSent>, WalletError> {
    let why = said(recheck_batch(tor, from, txs)?);
    Ok(why.map(|why| ShieldSplitSent { sent: Vec::new(), stopped: Some(why.into()) }))
}

fn said(recheck: Recheck) -> Option<&'static str> {
    match recheck {
        Recheck::Fits => None,
        Recheck::ShortOfEth => Some(GAS_EATS),
        Recheck::Moved => Some(FEES_MOVED),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_gas_rise_the_eth_cannot_pay_says_the_reviews_own_line() {
        assert_eq!(said(Recheck::ShortOfEth), Some(GAS_EATS));
        assert_eq!(said(Recheck::Moved), Some(FEES_MOVED));
        assert_eq!(said(Recheck::Fits), None);
    }
}
