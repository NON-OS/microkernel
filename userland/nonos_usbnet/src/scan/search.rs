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

//! One pass over every known host's root ports, fastest host first, until a
//! port holds this driver's device. Each host keeps its own port book, so
//! port 1 of one host is not port 1 of another. Pure over the calls to the
//! hosts, so the proofs run it with modelled hosts.

use alloc::vec::Vec;

use super::book::Book;
use super::port::Port;
use crate::hosts::Hosts;

/// What trying one port came to.
pub enum Tried<B> {
    Ours(B),
    NotOurs,
    Busy,
    Failed,
}

/// `ports(host)` lists a host's connected ports; `try_port(host, port)`
/// tries one.
pub fn search<B>(
    hosts: &mut Hosts<Book>,
    mut ports: impl FnMut(u32) -> Option<Vec<Port>>,
    mut try_port: impl FnMut(u32, u8) -> Tried<B>,
) -> Option<B> {
    hosts.first(|host, book| {
        let connected = ports(host)?;
        for port in book.plan(&connected) {
            match try_port(host, port) {
                Tried::Ours(bound) => return Some(bound),
                Tried::NotOurs => book.not_ours(port),
                Tried::Busy => {}
                Tried::Failed => book.failed(port),
            }
        }
        None
    })
}
