//! The market reader of a swap a fuzzer reaches: a batch of replies read as the market of each
//! route the wallet trades on mainnet, and as the state of the NOX token. Built only with `fuzzing`.

use super::{parse, parse_nox, route, MAINNET};
use crate::evm::network::Network;
use crate::evm::reply::batch;
use crate::net::asset::Coin;

pub(crate) fn market(text: &str) {
    let chain = Network::Mainnet.chain();
    let Ok(answers) = batch(text, 16) else { return };
    let _ = parse_nox::state(&chain, &answers);
    for (from, to) in [(Coin::Eth, Coin::Nox), (Coin::Nox, Coin::Usdc), (Coin::Usdc, Coin::Eth)] {
        let Some(hops) = route::route(&chain, &MAINNET, from, to) else { continue };
        let nox = from == Coin::Nox || to == Coin::Nox;
        let _ = parse::market(&chain, &hops, from != Coin::Eth, nox, &answers);
    }
}
