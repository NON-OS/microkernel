// NONOS Operating System (AGPL-3.0-or-later)
//! The anonymity network, as the phones' `net::tor` presents it.
//!
//! The phones run an embedded Tor. NONOS has its own networks, chosen once
//! for the whole machine: the Nym mixnet or the Anyone network. This module
//! keeps the phones' interface, `Tor`, `Purpose`, `TorStream` and `tls`, so
//! every caller above it is the phones' code unchanged, and carries each
//! stream over the chosen network through `nonos_route_link`. It never falls
//! back to a direct connection: a chosen network that is down refuses with
//! its reason, and so does Direct, which is not anonymous.
//!
//! What it cannot reach is a `.onion` address: neither Nym nor Anyone serves
//! Tor's onion services. A connection to one is refused at once, and the
//! wallet settles from the owner's own account, as the phones do when no
//! lander answers. An Anyone onion service, `.anyone`, is reached: the route
//! link sends it through net.anon whatever the default network is.

mod stream;
mod tls;

pub use stream::TorStream;
pub use tls::{tls, TlsStream};

use crate::error::NetError;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    Scan,
    Relay,
    Deposit,
    Mainnet,
    Sepolia,
}

/// The chosen network. Nothing is held open between streams.
pub struct Tor {
    _private: (),
}

impl Tor {
    /// Ready when the chosen network is anonymous and running.
    pub fn start(_dir: &Path) -> Result<Tor, NetError> {
        stream::anonymous_route()?;
        Ok(Tor { _private: () })
    }

    pub fn connect(&self, purpose: Purpose, host: &str, port: u16) -> Result<TorStream, NetError> {
        self.connect_within(purpose, (host, port), stream::STALL)
    }

    pub fn connect_within(
        &self,
        _purpose: Purpose,
        (host, port): (&str, u16),
        limit: std::time::Duration,
    ) -> Result<TorStream, NetError> {
        if host.ends_with(".onion") {
            return Err(NetError::ProxyRefused);
        }
        TorStream::open(host, port, limit)
    }

    /// Accounts share the one machine-wide network here; a stream is its own
    /// session either way.
    pub fn use_account(&self, _index: u32) {}
}

pub fn as_account<T>(_index: u32, work: impl FnOnce() -> T) -> T {
    work()
}
