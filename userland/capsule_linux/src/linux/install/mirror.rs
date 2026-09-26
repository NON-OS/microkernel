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


//! Where packages come from, by address.
//!
//! The installer never resolves a name, so an install sends nothing to a
//! resolver. The default is the address Alpine's CDN answered from when this
//! was written, reached with Alpine's name as the Host line. A build sets
//! NONOS_ALPINE_MIRROR to a.b.c.d:port to use another mirror; Alpine's own
//! signatures authenticate the bytes whichever mirror serves them.

const DEFAULT: &str = "151.101.66.132:80";

/// The mirror's address, as the socket service takes it, and its port.
pub(super) fn mirror() -> (&'static str, u16) {
    let at = option_env!("NONOS_ALPINE_MIRROR").unwrap_or(DEFAULT);
    let (ip, port) = at.rsplit_once(':').unwrap_or((at, "80"));
    (ip, port.parse().unwrap_or(80))
}

/// The name a CDN serves Alpine's tree under.
pub(super) const HOST_LINE: &str = "dl-cdn.alpinelinux.org";
