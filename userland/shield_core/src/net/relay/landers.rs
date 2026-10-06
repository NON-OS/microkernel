//! The landers of a pool, tried in order. One is used only when its `/v1/info` names the pool and
//! takes a fee paid to whoever submits. A lander that cannot be reached, or names another pool, is
//! passed over for the next. A refusal from one that answered is its verdict and ends the search.
//! A proof may reach more than one lander: the pool lands it once.

use super::{ask, hand, reply, Handed};
use crate::error::NetError;
use crate::net::tor::Tor;

/// The first lander in `landers` ready for `pool`, and what it made of the hand-off `json`.
pub fn hand_first(
    tor: &Tor,
    landers: &[&str],
    pool: &str,
    json: &str,
) -> Result<(usize, Handed), NetError> {
    let mut last = NetError::Transport;
    for (at, onion) in landers.iter().enumerate() {
        let tried = ready(tor, onion, pool).and_then(|_| hand(tor, onion, json));
        match tried {
            Ok(handed) => return Ok((at, handed)),
            Err(e) => last = e,
        }
    }
    Err(last)
}

/// Whether the lander at `onion` serves `pool` and takes the submitter fee, as one error if not.
fn ready(tor: &Tor, onion: &str, pool: &str) -> Result<(), NetError> {
    let (status, body) = ask(tor, onion, ("GET", "/v1/info"), None)?;
    let serves = reply::serves(&body, pool) && reply::takes_submitter_fee(&body);
    if status != 200 || !serves {
        return Err(NetError::WrongRelayer);
    }
    Ok(())
}

#[cfg(test)]
mod live {
    #![allow(clippy::expect_used)]
    use super::ready;
    use crate::net::pool::ACTIVE;
    use crate::net::tor::Tor;

    /// Every production lander answers over Tor for the production pool. Needs the network.
    #[test]
    #[ignore]
    fn every_lander_serves_the_pool_over_tor() {
        let tor = Tor::start(&std::env::temp_dir().join("nox-tor-landers")).expect("bootstrap");
        for onion in ACTIVE.landers {
            ready(&tor, onion, ACTIVE.address).expect(onion);
        }
    }
}
