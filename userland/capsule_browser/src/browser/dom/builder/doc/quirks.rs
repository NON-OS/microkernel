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

use crate::browser::html::tokenizer::Doctype;

use super::super::super::quirks::Quirks;
use super::{quirks_ietf, quirks_vendor};

const XHTML_LIMITED: [&str; 2] =
    ["-//w3c//dtd xhtml 1.0 frameset//", "-//w3c//dtd xhtml 1.0 transitional//"];
const HTML401: [&str; 2] =
    ["-//w3c//dtd html 4.01 frameset//", "-//w3c//dtd html 4.01 transitional//"];

/// The mode a doctype selects (13.2.6.4.1). Identifiers compare ignoring
/// ASCII case.
pub fn quirks_of(d: &Doctype) -> Quirks {
    if d.force_quirks || d.name.as_deref() != Some("html") {
        return Quirks::Full;
    }
    let public = d.public_id.as_deref().unwrap_or("");
    let has_public = d.public_id.is_some();
    let system = d.system_id.as_deref().unwrap_or("");
    let starts = |list: &[&str]| list.iter().any(|p| starts_ci(public, p));
    let exact =
        ["-//w3o//dtd w3 html strict 3.0//en//", "-/w3c/dtd html 4.0 transitional/en", "html"];
    let quirks = (has_public && exact.iter().any(|x| public.eq_ignore_ascii_case(x)))
        || system
            .eq_ignore_ascii_case("http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd")
        || (has_public && (starts(quirks_ietf::PREFIXES) || starts(quirks_vendor::PREFIXES)))
        || (system.is_empty() && has_public && starts(&HTML401));
    if quirks {
        return Quirks::Full;
    }
    if has_public && (starts(&XHTML_LIMITED) || (!system.is_empty() && starts(&HTML401))) {
        return Quirks::Limited;
    }
    Quirks::No
}

fn starts_ci(s: &str, prefix: &str) -> bool {
    s.len() >= prefix.len() && s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}
