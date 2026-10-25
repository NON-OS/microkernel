// NONOS Operating System (AGPL-3.0-or-later)
//! The routes Shield holds: Nym, Anyone or none, and no Direct to hold.

use super::Route;

/// Why a Direct handed in from route link is no route for Shield.
pub const NYM_OR_ANYONE: &str =
    "Shield reaches the network only over Nym or Anyone, never by a direct connection";

/// Every route a Shield stream is opened from. Route link's Direct has no
/// case here, so no Shield code can take or hand one on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShieldRoute {
    /// Through net.socks5, at this port.
    Nym(u32),
    /// Through net.anon, at this port.
    Anon(u32),
    /// No route Shield may take, and why.
    Down(&'static str),
}

impl ShieldRoute {
    /// The one way in from route link. A Direct handed in is no route: it
    /// opens nothing and Shield waits for Nym or Anyone.
    pub fn from_link(route: Route) -> ShieldRoute {
        match route {
            Route::Nym(port) => ShieldRoute::Nym(port),
            Route::Anon(port) => ShieldRoute::Anon(port),
            Route::Down(why) => ShieldRoute::Down(why),
            Route::Direct => ShieldRoute::Down(NYM_OR_ANYONE),
        }
    }

    /// The route link's stream APIs take, or None when there is no route.
    pub fn link(self) -> Option<Route> {
        match self {
            ShieldRoute::Nym(port) => Some(Route::Nym(port)),
            ShieldRoute::Anon(port) => Some(Route::Anon(port)),
            ShieldRoute::Down(_) => None,
        }
    }
}
