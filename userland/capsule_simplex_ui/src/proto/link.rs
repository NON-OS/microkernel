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

//! Connection links in an answer. Since v6.3 the core sends a `CreatedConnLink`
//! (`connFullLink`, and `connShortLink` when one was made); before, a bare
//! `connReq...` string. The short link is the one a person shares.

use alloc::string::String;

use super::json::Value;

fn created(link: &Value) -> Option<&str> {
    link.get("connShortLink").str().or(link.get("connFullLink").str())
}

/// The link under `new_key` (a `CreatedConnLink`) or `old_key` (a string).
pub fn link_in(v: &Value, new_key: &str, old_key: &str) -> Option<String> {
    created(v.get(new_key)).or(v.get(old_key).str()).map(String::from)
}

/// The address in a `userContactLink` answer.
pub fn address(resp: &Value) -> Option<String> {
    link_in(resp.get("contactLink"), "connLinkContact", "connReqContact")
}
