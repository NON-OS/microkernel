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

//! The words for the headline, a network and a transport's stage.

use super::super::session::Headline;
use nonos_route_proof::{Network, Stage};

pub fn headline(h: Headline) -> &'static [u8] {
    match h {
        Headline::AttestedAndAnonymous(Network::Nym) => {
            b"Attested, and anonymous over the Nym mixnet"
        }
        Headline::AttestedAndAnonymous(Network::Anyone) => {
            b"Attested, and anonymous over the Anyone network"
        }
        Headline::AttestedNotAnonymous => {
            b"Attested, not anonymous: the direct route shows this machine"
        }
        Headline::AttestedRouteDown => {
            b"Attested; the anonymity route is not up, so nothing leaves"
        }
        Headline::NotAttested => b"Not attested: a check below failed",
        Headline::Unknown => b"Not established: something below could not be read",
    }
}

pub fn network(n: Network) -> &'static [u8] {
    match n {
        Network::Nym => b"Nym mixnet",
        Network::Anyone => b"Anyone network",
    }
}

pub fn stage(s: Stage) -> &'static [u8] {
    match s {
        Stage::Cold => b"not started",
        Stage::Bootstrapping => b"fetching the directory",
        Stage::Joining => b"directory verified, building the route",
        Stage::Ready => b"up",
        Stage::Failed => b"stopped on an error",
    }
}
