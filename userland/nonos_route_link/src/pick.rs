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

/*
 * Which network a connection leaves through, decided from the system's
 * default network and from which anonymity networks run, and from nothing
 * else, so the rule is held on the host (route_link_proofs, and
 * model_fetch_proofs for the fetcher that first carried it). The default is
 * the one setup asks for and Settings changes, and the Nym mixnet when the
 * policy store holds a value this build does not know or cannot be asked,
 * as the browser reads it. A default whose network is not running is no
 * route. Nothing reverts to another network, and never to a direct
 * connection, which names this machine to the far end: that is taken only
 * when the default says so.
 */

use nonos_policy_proto::route;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    /* Through net.socks5, at this port. */
    Nym(u32),
    /* Through net.anon, at this port. */
    Anon(u32),
    Direct,
    /// The default names a network that does not run on this boot.
    Down(&'static str),
}

pub const NYM_DOWN: &str = "the Nym mixnet, the default, is not running";
pub const ANYONE_DOWN: &str = "the Anyone network, the default, is not running";
pub const UNREAD: &str = "the default network could not be read and the Nym mixnet is not running";
pub const ANYONE_NEEDED: &str =
    "an .anyone address is reached only inside the Anyone network, which is not running";

/*
 * The route a download for an install takes, a Qwen tier's model or a
 * Linux package: the Anyone network, whatever network the browser uses
 * (the default the person chose stays the browser's). Nym's exits rotate
 * and end a long stream part way; Anyone's circuits carry gigabytes. A
 * direct download is taken only when the person picks it for that download
 * (the fetcher's `--direct`). `anon` is net.anon's port, 0 when it has not
 * registered; then there is no route yet, and the caller waits for it.
 */
pub fn install_route(anon: u32) -> Route {
    match anon {
        0 => Route::Down(ANYONE_INSTALLS_DOWN),
        port => Route::Anon(port),
    }
}

pub const ANYONE_INSTALLS_DOWN: &str =
    "the Anyone network, which downloads installs, is not running yet";

/* An Anyone onion service's name: anything ending in ".anyone", in any case,
 * with any number of trailing dots. net.anon checks the rest and refuses a
 * malformed one, so no such name is ever handed to an exit. */
pub fn is_anyone(host: &str) -> bool {
    const SUFFIX: &[u8] = b".anyone";
    let h = host.trim_end_matches('.').as_bytes();
    h.len() > SUFFIX.len() && h[h.len() - SUFFIX.len()..].eq_ignore_ascii_case(SUFFIX)
}

/* A full onion address is 56 letters before ".anyone"; anything shorter
 * is a short name, which net.anon looks up in the list the Anyone DNS
 * services sign. */
const ONION_LEN: usize = 56;

pub fn is_short_anyone(host: &str) -> bool {
    is_anyone(host) && host.trim_end_matches('.').len() - ".anyone".len() != ONION_LEN
}

/*
 * The route a wallet's chain reads and broadcasts take: never Direct, since
 * the RPC host would see this machine's address beside the account it asks
 * about. The default holds when it is Nym or Anyone; a Direct default goes
 * over Nym (`nym`, the port of net.socks5) or else Anyone (`anon`), and with
 * neither running there is no route, said as such, and nothing is read.
 */
pub fn private_only(route: Route, nym: u32, anon: u32) -> Route {
    match (route, nym, anon) {
        (Route::Direct, 0, 0) => Route::Down(PRIVATE_DOWN),
        (Route::Direct, 0, port) => Route::Anon(port),
        (Route::Direct, port, _) => Route::Nym(port),
        (other, _, _) => other,
    }
}

pub const PRIVATE_DOWN: &str = "the wallet reads the chain only over Nym or Anyone, and \
     neither is running";

/* What a screen says wherever it shows a short name, as net.anon words it. */
pub const SHORT_NOTICE: &str = "Reached by a short .anyone name from the list the Anyone DNS \
     services sign. A short name is weaker than the full address: the list decides where it \
     points. NONOS refuses a name that changes service within a boot.";

/*
 * The route for a connection to `host`. An .anyone service lives inside the
 * Anyone network and nothing else reaches it, so it goes through net.anon
 * (port `anon`, 0 when not running) whatever the default is: the stream
 * never leaves through an exit and never names this machine, so this is
 * never weaker than the default. Every other host keeps `route`.
 */
pub fn for_host(route: Route, host: &str, anon: u32) -> Route {
    match (is_anyone(host), anon) {
        (false, _) => route,
        (true, 0) => Route::Down(ANYONE_NEEDED),
        (true, port) => Route::Anon(port),
    }
}

/*
 * The route for `default`, the policy store's answer (None when it could
 * not be asked), with `nym` and `anon` the ports of net.socks5 and net.anon,
 * 0 for one that is not running.
 */
pub fn pick(default: Option<u8>, nym: u32, anon: u32) -> Route {
    match default {
        Some(route::DIRECT) => Route::Direct,
        Some(route::ANYONE) if anon != 0 => Route::Anon(anon),
        Some(route::ANYONE) => Route::Down(ANYONE_DOWN),
        _ if nym != 0 => Route::Nym(nym),
        Some(_) => Route::Down(NYM_DOWN),
        None => Route::Down(UNREAD),
    }
}
