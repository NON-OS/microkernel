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
//! Run a nox network command; returns whether it succeeded.

use super::route::net::Net;
use super::{http, ifconfig, nslookup, nym, ping};
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Net, args: &[&[u8]], rest: &[&[u8]]) -> bool {
    match cmd {
        Net::Ifconfig => ifconfig::run(state),
        Net::Nslookup => nslookup::run(state, rest),
        Net::Nym => nym::run(state),
        Net::Http => http::run(state, rest),
        Net::Ping => {
            ping::run(state, args);
            true
        }
    }
}
