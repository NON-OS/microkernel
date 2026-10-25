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


//! Where Alpine's packages come from: Alpine's CDN, by name. The name is
//! resolved by the network the install leaves through (http/route.rs), never
//! in the clear unless Direct is the person's choice. A development build
//! (dev-mirror) takes the host's relay instead, at the address QEMU's user
//! network gives the host, dialled directly (docs/build/dev-mirror.md). No
//! setting moves either: build.rs refuses NONOS_ALPINE_MIRROR in every
//! build. Alpine's own signatures and the market's pin authenticate the
//! bytes whichever mirror serves them.

#[cfg(not(feature = "dev-mirror"))]
const AT: &str = "dl-cdn.alpinelinux.org:80";
#[cfg(feature = "dev-mirror")]
const AT: &str = "10.0.2.2:8080";

/// The name a CDN serves Alpine's tree under, the Host line for a mirror
/// given by address.
pub const HOST_LINE: &str = "dl-cdn.alpinelinux.org";

/// The mirror this build fetches from: its name or address, its port, and
/// the Host line it is asked with.
pub fn mirror() -> (&'static str, u16, &'static str) {
    parse(AT)
}

/// `name:port` or `a.b.c.d:port`, the port 80 when absent or unreadable. A
/// mirror is asked by its own name; one given by address is asked as
/// Alpine's CDN.
pub fn parse(at: &str) -> (&str, u16, &str) {
    let (name, port) = at.rsplit_once(':').unwrap_or((at, "80"));
    let host = match name.split('.').all(|o| o.parse::<u8>().is_ok()) {
        true => HOST_LINE,
        false => name,
    };
    (name, port.parse().unwrap_or(80), host)
}
