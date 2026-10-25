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

//! A connect by host run the way a guest's goes: the capsule's own encoder,
//! net.sockets' own reader and its decision, counting what net.dns is asked.

use crate::host_body::host_body;
use crate::server::handlers::host_target::host_target;
use crate::server::handlers::parse_host::parse;

/// Run a connect by host the way a guest's goes, counting what net.dns is
/// asked.
pub(super) fn connect_by_host(
    mixnet: bool,
    host: &[u8],
    answer: Result<[u8; 4], u16>,
) -> (Result<[u8; 4], u16>, usize) {
    connect_under(true, mixnet, host, answer)
}

/// The same, with the chosen network Direct or not.
pub(super) fn connect_under(
    direct: bool,
    mixnet: bool,
    host: &[u8],
    answer: Result<[u8; 4], u16>,
) -> (Result<[u8; 4], u16>, usize) {
    let body = host_body(9, 443, host).unwrap_or_default();
    let Some((_, _, name)) = parse(&body) else { panic!("the server reads what the capsule sent") };
    let mut asked = 0usize;
    let got = host_target(
        mixnet,
        name,
        || direct,
        |_| {
            asked += 1;
            answer
        },
    );
    (got, asked)
}
