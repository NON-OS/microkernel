// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Which way each connection leaves, decided for its host.
//!
//! The browser used to hold one route for everything, the reader's network,
//! so an .anyone address went to the Nym mixnet (whose exits cannot reach
//! it) or, with Direct chosen, to net.dns in the clear. A connection now
//! takes its way from its own host, by the rule every other program uses
//! (`nonos_route_link::for_host`): an .anyone service lives inside the
//! Anyone network and goes through net.anon whatever the choice, or is
//! refused when net.anon is not running. That is never weaker than the
//! choice: the stream never leaves through an exit.
//!
//! A page is one network. Everything a page reached at an .anyone address
//! asks for leaves through Anyone, its clearnet images and scripts too: a
//! page that is an onion service must not have the reader's address handed
//! to a clearnet host by a direct fetch for one of its images. Any other
//! page's connections leave through the reader's network, an .anyone host
//! among them still going through net.anon. Direct is taken only when the
//! page's network is Direct, and nothing ever falls back to it.
//!
//! The routes are set at each navigation (`Routes::for_page`) and every
//! connection of that page takes its way from them when it opens, so a page
//! that follows cannot reuse a way the one before it chose.
//!
//! Pure; the proofs hold it.

use nonos_route_link::{for_host, is_anyone, is_short_anyone, Proxy, Route};

use super::choice::Network;

/// How one connection leaves.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Way {
    /// Through net.sockets, naming this machine to the far end.
    Direct,
    /// Through a proxy's SOCKS front: the network, the proxy's port, and
    /// which of its ways (for what its refusals mean).
    Proxy { net: Network, port: u32, proxy: Proxy },
    /// Not at all, and why.
    Refused(&'static str),
}

impl Way {
    pub fn proxied(self) -> bool {
        matches!(self, Way::Proxy { .. })
    }

    /// The network the connection rides, which sets how long it may wait.
    pub fn network(self) -> Network {
        match self {
            Way::Proxy { net, .. } => net,
            Way::Direct | Way::Refused(_) => Network::Direct,
        }
    }

    /// What a refused SOCKS CONNECT means on this way, from its reply code.
    /// An .anyone service is told apart from an exit: code 4 there means no
    /// descriptor answered, or, for a short name, that the name is not in
    /// the signed list (route_link's words, as the terminal says them).
    pub fn refused(self, rep: u8) -> &'static str {
        match self {
            Way::Proxy { proxy: p @ (Proxy::AnyoneService | Proxy::AnyoneName), .. } => {
                p.refused(rep)
            }
            Way::Proxy { net, .. } => net.refused(rep),
            Way::Direct | Way::Refused(_) => "socks connect rejected",
        }
    }
}

/// The ways a page's connections may take: the page's network, and the
/// ports of net.socks5 and net.anon, 0 for one that is not running.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Routes {
    pub page: Network,
    pub nym: u32,
    pub anon: u32,
}

impl Routes {
    /// The routes of a page reached at `host` while the reader has chosen
    /// `chosen`.
    pub fn for_page(host: &str, chosen: Network, nym: u32, anon: u32) -> Routes {
        let page = if is_anyone(host) { Network::Anyone } else { chosen };
        Routes { page, nym, anon }
    }

    /// The way a connection to `host` takes from this page.
    pub fn way(&self, host: &str) -> Way {
        let route = match self.page {
            Network::Direct => Route::Direct,
            Network::Nym if self.nym != 0 => Route::Nym(self.nym),
            Network::Anyone if self.anon != 0 => Route::Anon(self.anon),
            n => Route::Down(n.absent()),
        };
        match for_host(route, host, self.anon) {
            Route::Direct => Way::Direct,
            Route::Nym(port) => Way::Proxy { net: Network::Nym, port, proxy: Proxy::Nym },
            Route::Anon(port) => Way::Proxy { net: Network::Anyone, port, proxy: anyone(host) },
            Route::Down(why) => Way::Refused(why),
        }
    }

    /// Whether anything may leave direct now, the reader having `chosen`: a
    /// page whose network is Direct, and a reader who has not since chosen
    /// another. A fetch the page before opened direct is refused the moment
    /// the reader moves to an anonymous network.
    pub fn direct(&self, chosen: Network) -> bool {
        self.page == Network::Direct && chosen == Network::Direct
    }
}

/// Which of net.anon's ways serves `host`.
fn anyone(host: &str) -> Proxy {
    match (is_anyone(host), is_short_anyone(host)) {
        (true, true) => Proxy::AnyoneName,
        (true, false) => Proxy::AnyoneService,
        (false, _) => Proxy::Anyone,
    }
}
