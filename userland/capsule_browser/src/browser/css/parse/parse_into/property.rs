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
use alloc::vec::Vec;

use crate::browser::css::decl::Decl;
use crate::browser::css::rule::Rule;

use super::super::decls::parse_decls;
use super::cx::Cx;

/* @property --name { syntax: ...; inherits: ...; initial-value: ...; }:
 * a data rule whose first decl names the property and the rest are its
 * descriptors, read by the cascade's custom-property registry. */
pub(super) fn register(name: &str, body: &str, cx: &mut Cx) {
    let name = name.trim();
    if !name.starts_with("--") || name.contains(char::is_whitespace) {
        return;
    }
    let mut decls: Vec<Decl> = Vec::new();
    decls.push(Decl::new(String::from("property"), name.to_ascii_lowercase(), false));
    decls.extend(parse_decls(body));
    cx.statement(decls, Rule::PROPERTY);
}
