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
//! Usage of the kernel benchmark and the commands that reach the network.

pub(crate) const NET: &[(&[u8], &[u8], &[u8])] = &[
    (b"bench", b"bench", b"cycle costs of the kernel primitives, as percentiles"),
    (b"ifconfig", b"ifconfig", b"interfaces, addresses and link state"),
    (b"nslookup", b"nslookup <name>", b"resolve a name through the configured resolver"),
    (
        b"curl",
        b"curl <url>",
        b"fetch a URL over the chosen network; http, get and fetch are the same command",
    ),
    (b"nym", b"nym", b"mixnet client state: directory, gateway and route"),
];
