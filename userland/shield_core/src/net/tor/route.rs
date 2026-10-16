// NONOS Operating System (AGPL-3.0-or-later)
//! The route a stream takes: the chosen network, refused unless it is anonymous and up.

use crate::error::NetError;

#[cfg(target_vendor = "nonos")]
use nonos_route_link::Route;

/// The chosen route, refused unless it is anonymous and up.
#[cfg(target_vendor = "nonos")]
pub(super) fn anonymous_route() -> std::result::Result<Route, NetError> {
    let route = Route::chosen();
    match route {
        Route::Nym(_) | Route::Anon(_) => Ok(route),
        Route::Direct | Route::Down(_) => Err(NetError::ProxyUnreachable),
    }
}

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn anonymous_route() -> std::result::Result<(), NetError> {
    Err(NetError::ProxyUnreachable)
}
