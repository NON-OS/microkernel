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

use alloc::string::String;

use super::authority::authority;
use super::types::{Scheme, Url};

/* The address as the browser shows and records it: scheme, host (with the
 * port when it is not the scheme's default) and the path with its query
 * and fragment, the same form `join` builds, so the two compare equal. */
pub fn to_string(url: &Url) -> String {
    let scheme = if url.scheme == Scheme::Https { "https" } else { "http" };
    alloc::format!("{}://{}{}", scheme, authority(url), url.path)
}
