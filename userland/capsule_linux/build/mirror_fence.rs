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

//! Which mirror settings a build may compile in (docs/build/dev-mirror.md).
//! A mirror on the local network is dialled directly, past nonos_route_link,
//! and only a development build (`dev-mirror`) carries that path. So no
//! other build may name one, and NONOS_ALPINE_MIRROR, which only development
//! images set, needs the feature whatever it names.

use super::route::is_local;

/// The mirror settings whose host part could name a local address.
pub const MIRRORS: [&str; 4] =
    ["NONOS_ALPINE_MIRROR", "NONOS_DEB_MIRROR", "NONOS_PACMAN_MIRROR", "NONOS_PACKAGE_MIRROR"];

/// Why a build with these settings is refused, or None. `get` reads a
/// setting; `dev` is whether the build has the dev-mirror feature.
pub fn refused(get: impl Fn(&str) -> Option<String>, dev: bool) -> Option<String> {
    if dev {
        return None;
    }
    for var in MIRRORS {
        let Some(at) = get(var).filter(|v| !v.is_empty()) else { continue };
        if var == "NONOS_ALPINE_MIRROR" {
            return Some(format!(
                "{var}={at}: only a development image sets it; build with dev-mirror"
            ));
        }
        let host = at.rsplit_once(':').map_or(at.as_str(), |(h, _)| h);
        if is_local(host) {
            return Some(format!(
                "{var}={at}: a mirror on the local network is development only (dev-mirror)"
            ));
        }
    }
    None
}
