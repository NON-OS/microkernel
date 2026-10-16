// NONOS Operating System (AGPL-3.0-or-later)
//! The route a stream takes: Shield's own route, refused unless it is Nym or Anyone and up.

use crate::error::NetError;

#[cfg(target_vendor = "nonos")]
use super::shield_route::ShieldRoute;
#[cfg(target_vendor = "nonos")]
use nonos_route_link::Route;

#[cfg(target_vendor = "nonos")]
impl ShieldRoute {
    /// Route link's `for_wallet` (the default when it is Nym or Anyone; a
    /// Direct default over Nym, else Anyone, else no route), held in
    /// Shield's own type, which has no Direct.
    pub(super) fn for_shield() -> ShieldRoute {
        ShieldRoute::from_link(Route::for_wallet())
    }
}

/// The route handed to route link for one stream: Nym or Anyone, refused
/// when Shield has no route.
#[cfg(target_vendor = "nonos")]
pub(super) fn anonymous_route() -> std::result::Result<Route, NetError> {
    ShieldRoute::for_shield().link().ok_or(NetError::ProxyUnreachable)
}

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn anonymous_route() -> std::result::Result<(), NetError> {
    Err(NetError::ProxyUnreachable)
}
