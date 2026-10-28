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

//! The route a stream takes, read when its connect is made and again each
//! second while the policy store or the network's service is not there
//! yet (guest_route.rs). A default the store does not give is waited for,
//! as a network that has not come up is, and then refused: it never
//! becomes the Nym mixnet.

use nonos_app_skeleton::discover::lookup_service;
use nonos_route_link::default_route;

use super::super::guest_route::{path, Net, Path};
use super::entry::{Entry, Phase};
use super::nym::begin;
use super::outcome::{fail, Outcome};
use super::retry;
use crate::linux::abi::errno::{ENETUNREACH, ENOBUFS};

const HELD: &str = "this family holds a model, and nothing it runs reaches a network";
const NO_STREAM: &str = "this capsule holds as many streams as net.socks5 gives one program";

/// Read the route and take the phase it names; Some when that ends the
/// step: a wait, or a refusal. `spare` is a named stream of net.socks5 no
/// entry holds.
pub fn route(e: &mut Entry, now: i64, spare: Option<u32>) -> Option<Outcome> {
    if crate::linux::file::models::held() {
        return Some(fail(e, ENETUNREACH, HELD));
    }
    let port_of = |name: &[u8]| lookup_service(name).map_or(0, |s| s.port);
    match path(default_route(), port_of(b"net.socks5"), port_of(b"net.anon")) {
        Path::Wait(net, why) => {
            e.why = (net, why);
            Some(match retry::again(e.until, now) {
                Some(at) => {
                    e.phase = Phase::Route { again: at };
                    Outcome::Same
                }
                None => Outcome::Failed(ENETUNREACH),
            })
        }
        Path::Anyone(port) => {
            e.why.0 = Net::Anyone;
            e.phase = Phase::Anyone { port, again: now };
            None
        }
        Path::Nym(port) => {
            e.why.0 = Net::Nym;
            let Some(stream) = spare else {
                return Some(fail(e, ENOBUFS, NO_STREAM));
            };
            match begin(port, &e.dest, stream) {
                Ok(opening) => e.phase = Phase::Nym { opening, stream },
                Err(why) => return Some(fail(e, ENETUNREACH, why)),
            }
            None
        }
    }
}
