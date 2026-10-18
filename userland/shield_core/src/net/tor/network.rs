// NONOS Operating System (AGPL-3.0-or-later)
//! The chosen network as the phones' `Tor`: nothing held open, one stream per connection.

use super::{route, stream, Purpose, TorStream};
use crate::error::NetError;
use std::path::Path;

/// The chosen network. Nothing is held open between streams.
pub struct Tor {
    _private: (),
}

impl Tor {
    /// Ready when the chosen network is anonymous and running.
    pub fn start(_dir: &Path) -> Result<Tor, NetError> {
        route::anonymous_route()?;
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
