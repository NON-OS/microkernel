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

use super::host_port::host_port;
use super::normalize_path::normalize_path;
use super::scheme_rest::scheme_rest;
use super::split_path::split_path;
use super::types::Url;

pub fn parse(input: &str) -> Option<Url> {
    let s = input.trim();
    let (scheme, rest) = scheme_rest(s)?;
    if rest.is_empty() {
        return None;
    }
    let (authority, path) = split_path(rest);
    let (host, port) = host_port(authority, scheme)?;
    Some(Url { scheme, host, port, path: normalize_path(path) })
}

/// Why `input` cannot be loaded: an address of another scheme (file://,
/// ftp://) is not a web address, which this browser does not fetch; any
/// other text it could not read at all (fetch/words.rs says each).
pub fn refusal(input: &str) -> &'static str {
    let s = input.trim();
    let other = s.find("://").is_some_and(|i| i > 0 && scheme_rest(s).is_none());
    if other {
        "not a web address"
    } else {
        "bad url"
    }
}
