// NONOS Operating System (AGPL-3.0-or-later)
//! Opening a stream over the chosen route, held to the route's patience.

use super::TorStream;
use crate::error::NetError;
use std::time::Duration;

#[cfg(target_vendor = "nonos")]
use super::super::route::anonymous_route;
#[cfg(target_vendor = "nonos")]
use nonos_route_link::RouteStream;

impl TorStream {
    #[cfg(target_vendor = "nonos")]
    pub(in crate::net::tor) fn open(
        host: &str,
        port: u16,
        limit: Duration,
    ) -> std::result::Result<Self, NetError> {
        let route = anonymous_route()?;
        let inner = RouteStream::connect(route, host, port).map_err(|_| NetError::Transport)?;
        let patience =
            Duration::from_millis(route.patience_ms()).min(limit).max(Duration::from_secs(1));
        Ok(TorStream { inner, patience })
    }

    #[cfg(not(target_vendor = "nonos"))]
    pub(in crate::net::tor) fn open(
        _host: &str,
        _port: u16,
        _limit: Duration,
    ) -> std::result::Result<Self, NetError> {
        Err(NetError::ProxyUnreachable)
    }
}
