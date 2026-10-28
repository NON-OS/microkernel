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

//! One step of a stream: the route read when it is due (route.rs), one try
//! or one slice of opening on the network it names, or the bytes of an
//! open one carried. Each call to a service waits a bounded time, so a step
//! never holds the serve loop long. What the step came to is applied to
//! the socket by drive.rs; nothing here reaches the socket table.

use nonos_route_link::{Proxy, Slice};

use super::anyone::{try_open, Tried};
use super::entry::{Entry, Phase};
use super::link::Link;
use super::nym::SLICE_MS;
use super::outcome::{fail, Outcome};
use super::{nym_errno::errno_of, retry};
use crate::linux::abi::errno::ENETUNREACH;

/// `spare` is a named stream of net.socks5 no entry holds.
pub fn step(e: &mut Entry, now: i64, spare: Option<u32>) -> Outcome {
    if let Phase::Route { again } = e.phase {
        if now < again {
            return Outcome::Same;
        }
        if let Some(out) = super::route::route(e, now, spare) {
            return out;
        }
    }
    match &mut e.phase {
        Phase::Anyone { port, again } if now >= *again => match try_open(*port, &e.dest) {
            Tried::Opened(id) => Outcome::Anon { port: *port, id },
            Tried::Refused(errno, why) => fail(e, errno, why),
            Tried::NotReady(why) => {
                e.why.1 = why;
                retry::again(e.until, now).map_or(Outcome::Failed(ENETUNREACH), |at| {
                    *again = at;
                    Outcome::Same
                })
            }
        },
        Phase::Nym { opening, .. } => match opening.step(SLICE_MS) {
            Slice::Waiting if now < e.until => Outcome::Same,
            Slice::Waiting => fail(e, ENETUNREACH, Proxy::Nym.slow()),
            Slice::Failed(why) => fail(e, errno_of(why), why),
            Slice::Done => {
                if let Phase::Nym { opening, stream } =
                    core::mem::replace(&mut e.phase, Phase::Closed)
                {
                    e.phase = Phase::Open(Link::new(stream, opening.into_stream()));
                }
                Outcome::Opened
            }
        },
        Phase::Open(link) => {
            link.pump();
            Outcome::Same
        }
        _ => Outcome::Same,
    }
}
