//! Which recent sends from this device are still waiting, asked of the relays.
//! A send any relay holds counts as waiting. A relay that does not answer fails
//! the review, since a send it holds but could not report would reuse its nonce.
//! Only sends at or past the chain's pending count are asked about.

use super::network::Chain;
use super::rpc::{ask_via, Call};
use crate::error::NetError;
use crate::net::tor::Tor;

/// Flashbots takes eight calls in a batch, and the chain id is one of them.
const PER_BATCH: usize = 4;

/// One past the highest nonce among `sends` a relay still holds, or zero.
pub(super) fn floor(
    tor: &Tor,
    chain: &Chain,
    sends: &[(u64, [u8; 32])],
    pending: u64,
) -> Result<u64, NetError> {
    let open: Vec<&(u64, [u8; 32])> = sends.iter().filter(|(n, _)| *n >= pending).collect();
    let mut floor = 0u64;
    for host in chain.send_rpcs {
        for chunk in open.chunks(PER_BATCH) {
            let calls: Vec<Call> = chunk.iter().map(|(_, h)| lookup(h)).collect();
            let answers = ask_via(tor, chain, &[host], &calls)?;
            for ((nonce, _), answer) in chunk.iter().zip(answers) {
                if !answer.result()?.starts_with("null") {
                    floor = floor.max(nonce.saturating_add(1));
                }
            }
        }
    }
    Ok(floor)
}

fn lookup(hash: &[u8; 32]) -> Call {
    let hex: String = hash.iter().map(|b| format!("{b:02x}")).collect();
    Call { method: "eth_getTransactionByHash", params: format!(r#"["0x{hex}"]"#) }
}

#[cfg(test)]
mod live {
    #![allow(clippy::expect_used, clippy::indexing_slicing)]
    use super::floor;
    use crate::evm::Network;
    use crate::net::tor::Tor;

    /// A mined mainnet transaction counts as held and an unsent hash does not.
    /// It needs the network over Tor, so it runs only with `--ignored`.
    #[test]
    #[ignore]
    fn the_relays_report_a_real_send_and_not_an_invented_one() {
        let tor = Tor::start(&std::env::temp_dir().join("nox-tor-held")).expect("bootstrap");
        let chain = Network::Mainnet.chain();
        let mut real = [0u8; 32];
        let hex = "12b890e8b9a50ff5386932d6fe4b5159804eef7bfe2183b9982e55495b443cca";
        for (i, b) in real.iter_mut().enumerate() {
            *b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hex");
        }
        assert_eq!(floor(&tor, &chain, &[(7, real)], 0).expect("relays"), 8);
        assert_eq!(floor(&tor, &chain, &[(7, [0x5a; 32])], 0).expect("relays"), 0);
        assert_eq!(floor(&tor, &chain, &[(7, real)], 8).expect("relays"), 0, "below pending");
    }
}
