//! Every listed RPC of both networks, asked over Tor for its own chain.

use super::*;
use crate::evm::Network;

/// Every listed RPC of both networks, one at a time, over Tor. It needs the
/// network, so it runs only with `--ignored`.
#[test]
#[ignore]
fn every_listed_rpc_answers_over_tor_for_its_own_chain() {
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-rpcs")).expect("bootstrap");
    let body = body(&[]);
    let mut failed = Vec::new();
    for network in [Network::Mainnet, Network::Sepolia] {
        let chain = network.chain();
        for host in chain.rpcs.iter().chain(chain.send_rpcs) {
            if let Err(e) = once(&tor, &chain, host, &body, 0) {
                failed.push(format!("{network:?} {host}: {e:?}"));
            }
        }
    }
    assert!(failed.is_empty(), "RPCs that did not answer over Tor: {failed:?}");
}
